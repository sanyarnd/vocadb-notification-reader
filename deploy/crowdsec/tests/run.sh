#!/bin/sh
# Replays the Caddy logs from cases/ through CrowdSec and checks which of the
# vocadb-notification-reader scenarios fire. Runs inside the crowdsecurity/crowdsec image:
#
#   docker run --rm -v "$PWD/deploy/crowdsec:/vnr:ro" --entrypoint /vnr/tests/run.sh \
#     crowdsecurity/crowdsec:latest
#
# Set HUB_OFFLINE to a directory with s01-parse/ and s02-enrich/ parsers to skip the hub.
set -eu

ROOT=/vnr

mkdir -p /etc/crowdsec /var/lib/crowdsec/data
cp -a /staging/etc/crowdsec/. /etc/crowdsec/
cp -a /staging/var/lib/crowdsec/data/. /var/lib/crowdsec/data/

if [ -n "${HUB_OFFLINE:-}" ]; then
    for stage in s01-parse s02-enrich; do
        cp "$HUB_OFFLINE/$stage"/*.yaml "/etc/crowdsec/parsers/$stage/"
    done
else
    cscli hub update >/dev/null
    cscli collections install crowdsecurity/caddy >/dev/null
fi
cp "$ROOT"/scenarios/*.yaml /etc/crowdsec/scenarios/
cat "$ROOT/profiles.yaml" /staging/etc/crowdsec/profiles.yaml >/etc/crowdsec/profiles.yaml
# The agent always reports to a local API, register it with the embedded one.
cscli machines add --auto --force >/dev/null 2>&1

failed=0
for log in "$ROOT"/tests/cases/*.log; do
    name=$(basename "$log" .log)
    expected=$(sort -u "${log%.log}.expected")
    # In time-machine mode with the embedded API crowdsec doesn't exit on its own
    # after the replay, so stop it once it reports the shutdown.
    out=$(mktemp)
    crowdsec -c /etc/crowdsec/config.yaml -dsn "file://$log" -type caddy -no-capi >"$out" 2>&1 &
    pid=$!
    for _ in $(seq 1 120); do
        if grep -q "crowdsec shutdown" "$out" || ! kill -0 "$pid" 2>/dev/null; then
            break
        fi
        sleep 0.5
    done
    kill "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
    output=$(cat "$out")
    rm -f "$out"
    if ! echo "$output" | grep -q "crowdsec shutdown"; then
        echo "FAIL $name: crowdsec didn't process the log"
        echo "$output" | grep -v "level=info" | tail -20
        failed=1
        continue
    fi
    fired=$(echo "$output" |
        grep -o "performed '[^']*'" |
        sed "s/performed '\(.*\)'/\1/" |
        grep '^vocadb-notification-reader/' |
        sort -u || true)

    if [ "$fired" != "$expected" ]; then
        echo "FAIL $name: expected [${expected}], got [${fired}]"
        failed=1
    elif [ -n "$fired" ] && ! echo "$output" | grep -q ": 1h ban on Ip"; then
        echo "FAIL $name: the profile didn't apply a 1h ban"
        echo "$output" | grep " ban on " || true
        failed=1
    else
        echo "ok   $name${fired:+: $fired}"
    fi
done

exit $failed
