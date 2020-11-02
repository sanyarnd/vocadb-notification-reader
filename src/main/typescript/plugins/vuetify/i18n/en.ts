import { en } from "vuetify/src/locale";

export default {
  ...en,

  connectionError: "Connection error",
  unexpectedError: "Unexpected error",

  delete: "Delete",
  logout: "Logout",

  login: {
    username: "Username",
    password: "Password",
    usernameRequired: "Username is required",
    passwordRequired: "Password is required",
    loginWith: "Login with ",

    badCredentials: "Incorrect username or password"
  },

  notification: {
    search: "Search",
    type: {
      song: "Song",
      album: "Album",
      artist: "Artist",
      report: "Report",
      event: "Event",
      unknown: "Unknown"
    }
  },

  preferredLanguage: {
    original: "Original",
    romanized: "Romanized",
    english: "English",
    nonEnglish: "Non-English"
  }
};
