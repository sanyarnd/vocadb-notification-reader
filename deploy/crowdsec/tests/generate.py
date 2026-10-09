#!/usr/bin/env python3
"""Generates Caddy JSON access logs for the scenario tests (`python3 generate.py`)."""

import json
from pathlib import Path

START = 1_760_000_000  # fixed timestamp keeps the logs reproducible
HERE = Path(__file__).parent / "cases"


def line(ts, ip, method, uri, status, host="api.foobar.com"):
    return json.dumps(
        {
            "level": "info",
            "ts": ts,
            "logger": "http.log.access.log0",
            "msg": "handled request",
            "request": {
                "remote_ip": "10.10.0.2",
                "remote_port": "41234",
                "client_ip": ip,
                "proto": "HTTP/2.0",
                "method": method,
                "host": host,
                "uri": uri,
                "headers": {"User-Agent": ["Mozilla/5.0"]},
            },
            "bytes_read": 60,
            "duration": 0.25,
            "size": 80,
            "status": status,
            "resp_headers": {"Content-Type": ["application/json"]},
        }
    )


def case(name, lines, expected):
    (HERE / f"{name}.log").write_text("\n".join(lines) + "\n")
    (HERE / f"{name}.expected").write_text("".join(f"{s}\n" for s in expected))


def main():
    HERE.mkdir(exist_ok=True)
    login = ("POST", "/api/session")
    ip, other = "203.0.113.7", "198.51.100.1"

    # Ten failed logins within half a minute.
    case(
        "login-bf",
        [line(START + i * 3, ip, *login, 401) for i in range(10)],
        ["vocadb-notification-reader/login-bf"],
    )
    # Rate limited login attempts count as failures too.
    case(
        "login-bf-rate-limited",
        [line(START + i, ip, *login, 401) for i in range(5)]
        + [line(START + 5 + i, ip, *login, 429) for i in range(5)],
        ["vocadb-notification-reader/login-bf"],
    )
    # Someone mistyping a password a few times, then logging in.
    case(
        "login-typos",
        [line(START + i * 10, ip, *login, 401) for i in range(4)]
        + [line(START + 50, ip, *login, 200)],
        [],
    )
    # Failures spread over time leak out of the bucket.
    case(
        "login-slow",
        [line(START + i * 120, ip, *login, 401) for i in range(15)],
        [],
    )
    # Failures from different addresses (e.g. many users) are not added up.
    case(
        "login-distinct-ips",
        [line(START + i, f"198.51.100.{i}", *login, 401) for i in range(20)],
        [],
    )
    # Normal browsing and failed reads are not login failures.
    case(
        "browsing",
        [line(START + i, ip, "GET", "/api/notifications?offset=0&limit=25&language=Default", 200) for i in range(30)]
        + [line(START + 30 + i, ip, "GET", "/api/me", 401) for i in range(20)],
        [],
    )
    # A client hammering the API through 429.
    case(
        "rate-limit",
        [line(START + i * 0.1, other, "GET", "/api/me", 429) for i in range(40)],
        ["vocadb-notification-reader/rate-limit"],
    )
    # A short burst of 429 is tolerated.
    case(
        "rate-limit-burst",
        [line(START + i * 0.1, other, "GET", "/api/me", 429) for i in range(10)],
        [],
    )


if __name__ == "__main__":
    main()
