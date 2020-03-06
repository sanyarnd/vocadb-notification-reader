import { Actions, Getters, Module, Mutations } from "vuex-smart-module";
import { api } from "@/backend";
import { AccountData, AuthenticationPayload, PvService } from "@/backend/dto";

export const preferredLanguageData = ["DEFAULT", "JAPANESE", "ROMAJI", "ENGLISH"] as const;
export type PreferredLanguage = typeof preferredLanguageData[number];

export const itemsPerPageData = [10, 25, 50] as const;
export type ItemsPerPage = typeof itemsPerPageData[number];

class UserState {
  name = "";
  authenticated = false;
  itemsPerPage: ItemsPerPage = 25;
  preferredLanguage: PreferredLanguage = "DEFAULT";
  preferredService: PvService = "NicoNicoDouga";
}

class UserGetters extends Getters<UserState> {
  get name(): string {
    return this.state.name;
  }

  get isAuthenticated(): boolean {
    return this.state.authenticated;
  }

  get preferredLanguage(): PreferredLanguage {
    return this.state.preferredLanguage;
  }

  get preferredService(): PvService {
    return this.state.preferredService;
  }

  get itemsPerPage(): ItemsPerPage {
    return this.state.itemsPerPage;
  }
}

class UserMutations extends Mutations<UserState> {
  setAccountData(accountData: AccountData) {
    this.state.name = accountData.username;
    this.state.authenticated = true;
  }

  setPreferredLanguage(lang: PreferredLanguage) {
    this.state.preferredLanguage = lang;
  }

  setPreferredService(service: PvService) {
    this.state.preferredService = service;
  }

  setItemsPerPage(itemsPerPage: ItemsPerPage) {
    this.state.itemsPerPage = itemsPerPage;
  }
}

class UserActions extends Actions<UserState, UserGetters, UserMutations, UserActions> {
  public async authenticate(payload: AuthenticationPayload): Promise<void> {
    await api.authenticate(payload);

    const accountData = await api.accountData();
    this.mutations.setAccountData(accountData);
  }

  public async logout(): Promise<void> {
    localStorage.clear();
    await api.logout();
    window.location.href = "/login";
  }

  setPreferredLanguage(lang: PreferredLanguage) {
    this.mutations.setPreferredLanguage(lang);
  }

  setPreferredService(service: PvService) {
    this.mutations.setPreferredService(service);
  }

  setItemsPerPage(itemsPerPage: ItemsPerPage) {
    this.mutations.setItemsPerPage(itemsPerPage);
  }
}

export const userModule = new Module({
  state: UserState,
  getters: UserGetters,
  mutations: UserMutations,
  actions: UserActions
});
