export function authenticationExpireHandler(): void {
  if (window.location.pathname.startsWith("/login")) return;
  window.location.href = "/login";
}
