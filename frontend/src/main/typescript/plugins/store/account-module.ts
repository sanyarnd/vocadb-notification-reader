import { Actions, Getters, Module, Mutations } from "vuex-smart-module";
import { api } from "@/backend";
import { AccountData, AuthenticationPayload } from "@/backend/dto";

class AccountState {
  accountData: AccountData | null = null;
  accessToken: string | null = null;
}

class AccountGetters extends Getters<AccountState> {
  get accountData(): AccountData | null {
    return this.state.accountData;
  }

  get isAuthenticated(): boolean {
    return this.state.accessToken != null;
  }
}

class AccountMutations extends Mutations<AccountState> {
  setAccountData(accountData: AccountData | null) {
    this.state.accountData = accountData;
  }

  setAccessToken(token: string | null) {
    this.state.accessToken = token;
  }
}

class AccountActions extends Actions<
  AccountState,
  AccountGetters,
  AccountMutations,
  AccountActions
> {
  public async authenticate(payload: AuthenticationPayload): Promise<void> {
    const token = await api.authenticate(payload);
    this.mutations.setAccessToken(token.token);

    const accountData = await api.accountData();
    this.mutations.setAccountData(accountData);
  }

  public async logout(): Promise<void> {
    this.mutations.setAccessToken(null);
    this.mutations.setAccountData(null);
    window.location.href = "/login";
  }
}

export const accountModule = new Module({
  state: AccountState,
  getters: AccountGetters,
  mutations: AccountMutations,
  actions: AccountActions
});
