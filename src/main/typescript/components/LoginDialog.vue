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

    <v-select v-model="selectedLoginDatabase" :items="loginDatabases" solo>
      <template #selection="data">
        {{
          $vuetify.lang.t(`$vuetify.login.database.${data.item.toLowerCase()}`)
        }}
      </template>
      <template #item="data">
        {{
          $vuetify.lang.t(`$vuetify.login.database.${data.item.toLowerCase()}`)
        }}
      </template>
    </v-select>

    <v-btn :disabled="!valid" :loading="loginInProgress" block @click="submit">
      <v-avatar tile size="24px">
        <v-img src="../assets/login-icon.webp" />
      </v-avatar>
      {{
        $vuetify.lang.t("$vuetify.login.loginWith") +
        $vuetify.lang.t(
          `$vuetify.login.database.${selectedLoginDatabase.toLowerCase()}`
        )
      }}
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
import { accountModule } from "@/plugins/store/account-module";
import { LoginDatabase, loginDatabaseData } from "@/backend/dto";

@Component
export default class extends Vue {
  private accountStore = accountModule.context(this.$store);

  private readonly loginDatabases = loginDatabaseData;
  private selectedLoginDatabase: LoginDatabase = loginDatabaseData[0];

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

  private async submit() {
    this.loginInProgress = true;
    this.resetErrorState();

    try {
      await this.accountStore.actions.authenticate({
        username: this.username,
        password: this.password,
        database: this.selectedLoginDatabase
      });
      window.location.href = "/home";
    } catch (e) {
      this.showError = true;

      this.errorMessage = this.$vuetify.lang.t(
        e.response != undefined && e.response.status == 401
          ? "$vuetify.login.badCredentials"
          : "$vuetify.connectionError"
      );
    } finally {
      this.loginInProgress = false;
    }
  }
}
</script>
