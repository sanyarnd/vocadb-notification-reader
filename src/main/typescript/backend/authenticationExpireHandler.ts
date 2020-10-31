export function authenticationExpireHandler(): void {
  if (window.location.pathname.startsWith("/login")) return;

  localStorage.clear();
  window.location.href = "/login?redirect_url=" + window.location.href;
}
