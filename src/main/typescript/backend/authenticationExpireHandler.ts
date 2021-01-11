import {accountModule} from "@/plugins/store/account-module";
import {store} from "@/plugins/store";

export function authenticationExpireHandler(): void {
  if (window.location.pathname.startsWith("/login")) return;
  const userSettings = accountModule.context(store);
  userSettings.actions.logout();
}
