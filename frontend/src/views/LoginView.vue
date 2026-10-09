<script setup lang="ts">
import { ref, watch } from "vue";
import { useRouter } from "vue-router";
import { useLocale } from "vuetify";

import { isRateLimited, isUnauthorized } from "@/api/client";
import { type Database, databases } from "@/api/dto";
import { useAccountStore } from "@/stores/account";

const account = useAccountStore();
const router = useRouter();
const { t } = useLocale();

const valid = ref(false);
const username = ref("");
const password = ref("");
const database = ref<Database>(account.lastDatabase);
const showPassword = ref(false);
const loginInProgress = ref(false);
const errorMessage = ref<string | null>(null);

const required = (key: string) => (value: string) => value.length > 0 || t(key);

watch([username, password], () => (errorMessage.value = null));

function errorKey(error: unknown): string {
  if (isUnauthorized(error)) return "$vuetify.login.badCredentials";
  if (isRateLimited(error)) return "$vuetify.tooManyRequests";
  return "$vuetify.connectionError";
}

async function submit(): Promise<void> {
  if (!valid.value || loginInProgress.value) return;

  loginInProgress.value = true;
  errorMessage.value = null;
  try {
    await account.login({
      username: username.value,
      password: password.value,
      database: database.value
    });
    await router.push({ name: "home" });
  } catch (e) {
    errorMessage.value = t(errorKey(e));
  } finally {
    loginInProgress.value = false;
  }
}
</script>

<template>
  <v-row class="fill-height" justify="center" align="center">
    <v-col cols="12" sm="8" md="5" lg="4">
      <v-form v-model="valid" @submit.prevent="submit">
        <v-text-field
          v-model="username"
          name="username"
          autocomplete="username"
          :label="t('$vuetify.login.username')"
          :rules="[required('$vuetify.login.usernameRequired')]"
        />
        <v-text-field
          v-model="password"
          name="password"
          autocomplete="current-password"
          :label="t('$vuetify.login.password')"
          :rules="[required('$vuetify.login.passwordRequired')]"
          :type="showPassword ? 'text' : 'password'"
          :append-inner-icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'"
          @click:append-inner="showPassword = !showPassword"
        />
        <v-select
          v-model="database"
          name="database"
          variant="solo"
          :items="databases.map(value => ({ value, title: t(`$vuetify.database.${value}`) }))"
        />
        <v-btn type="submit" block :disabled="!valid" :loading="loginInProgress">
          {{ t("$vuetify.login.loginWith", t(`$vuetify.database.${database}`)) }}
        </v-btn>
        <v-alert v-if="errorMessage" class="mt-4" type="error" variant="tonal">
          {{ errorMessage }}
        </v-alert>
      </v-form>
    </v-col>
  </v-row>
</template>
