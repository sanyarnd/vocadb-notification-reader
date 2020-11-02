<template>
  <v-form v-model="valid">
    <v-text-field
      v-model="username"
      required
      :label="$vuetify.lang.t('$vuetify.login.username')"
      :rules="[usernameLength]"
      @keyup.enter="valid ? submit() : true"
    />
    <v-text-field
      v-model="password"
      required
      :label="$vuetify.lang.t('$vuetify.login.password')"
      :rules="[passwordLength]"
      :type="showPassword ? 'text' : 'password'"
      :append-icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'"
      @click:append="showPassword = !showPassword"
      @keyup.enter="valid ? submit() : true"
    />

    <v-select
      v-model="selectedPreferredLoginService"
      :items="loginServices"
      label="Login to..."
      solo
      @change="setPreferredLoginService"
    ></v-select>

    <v-btn :disabled="!valid" :loading="loginInProgress" block @click="submit">
      <v-avatar tile size="24px">
        <v-img src="../assets/login-icon.webp" />
      </v-avatar>
      {{ $vuetify.lang.t("$vuetify.login.loginVocaDB") }}
    </v-btn>
    <v-snackbar v-model="showError" color="red" transition="scale-transition">
      {{ errorMessage }}
      <v-btn text @click="showError = false">
        {{ $vuetify.lang.t("$vuetify.close") }}
      </v-btn>
    </v-snackbar>
  </v-form>
</template>

<script lang="ts">
import Vue from "vue";
import { Component, Watch } from "vue-property-decorator";
import { PreferredLoginService, userModule } from "@/plugins/store/user-module";

@Component
export default class extends Vue {
  private userStore = userModule.context(this.$store);

  private readonly loginServices: Array<PreferredLoginService> = [
    "VOCADB",
    "UTAITE",
    "TOUHOU"
  ];
  private selectedPreferredLoginService: PreferredLoginService = this.userStore
    .getters.preferredLoginService;

  private valid = false;

  private username = "";
  private password = "";
  private showPassword = false;
  private loginInProgress = false;

  private showError = false;
  private errorMessage = "";

  private usernameLength(name: string): boolean | string {
    return (
      name.length > 0 || this.$vuetify.lang.t("$vuetify.login.usernameRequired")
    );
  }

  private passwordLength(password: string): boolean | string {
    return (
      password.length > 0 ||
      this.$vuetify.lang.t("$vuetify.login.passwordRequired")
    );
  }

  @Watch("username")
  @Watch("password")
  private resetErrorState() {
    this.showError = false;
    this.errorMessage = "";
  }

  setPreferredLoginService(value: PreferredLoginService): void {
    this.userStore.actions.setPreferredLoginService(value);
  }

  private async submit() {
    this.loginInProgress = true;
    this.resetErrorState();

    try {
      await this.userStore.actions.authenticate({
        username: this.username,
        password: this.password,
        loginService: this.selectedPreferredLoginService
      });

      // if everything is ok, redirect
      const redirectUrl: string | null = new URLSearchParams(
        location.search
      ).get("redirect_url");
      window.location.href = redirectUrl ? redirectUrl : "/home";
    } catch (e) {
      this.showError = true;

      if (e.response != undefined && e.response.status == 401) {
        this.errorMessage = this.$vuetify.lang.t(
          "$vuetify.login.badCredentials"
        );
      } else {
        this.errorMessage = this.$vuetify.lang.t("$vuetify.connectionError");
      }
    } finally {
      this.loginInProgress = false;
    }
  }
}
</script>
