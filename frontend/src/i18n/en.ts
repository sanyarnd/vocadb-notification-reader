export const en = {
  tooManyRequests: "Too many attempts, try again later",
  connectionError: "Connection error",
  delete: "Delete",
  logout: "Logout",
  toDatabase: "Open in {0}",
  noData: "<no data>",

  login: {
    username: "Username",
    password: "Password",
    usernameRequired: "Username is required",
    passwordRequired: "Password is required",
    loginWith: "Login with {0}",
    badCredentials: "Incorrect username or password"
  },

  database: {
    VocaDb: "VocaDB",
    TouhouDb: "TouhouDB",
    UtaiteDb: "UtaiteDB"
  },

  notification: {
    search: "Search",
    title: "Title",
    artist: "Artist",
    releaseDate: "Release date",
    type: {
      song: "Song",
      album: "Album",
      artist: "Artist",
      report: "Report",
      event: "Event",
      unknown: "Unknown"
    },
    header: {
      type: "Type",
      songType: "Song type",
      title: "Title",
      artist: "Artist",
      tags: "Tags",
      releaseDate: "Release date",
      subject: "Subject",
      text: "Text"
    }
  },

  buttons: {
    itemsPerPage: "Items per page",
    theme: "Toggle theme"
  },

  preferredLanguage: {
    Default: "Original",
    Romaji: "Romanized",
    English: "English",
    Japanese: "Non-English"
  }
};

export type Messages = typeof en;
