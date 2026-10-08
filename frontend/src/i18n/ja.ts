import type { Messages } from "./en";

export const ja: Messages = {
  connectionError: "接続エラー",
  delete: "削除",
  logout: "ログアウト",
  toDatabase: "{0}で開く",
  noData: "<データなし>",

  login: {
    username: "ユーザー名",
    password: "パスワード",
    usernameRequired: "ユーザー名を入力してください",
    passwordRequired: "パスワードを入力してください",
    loginWith: "{0}でログイン",
    badCredentials: "ユーザー名またはパスワードが正しくありません"
  },

  database: {
    VocaDb: "VocaDB",
    TouhouDb: "TouhouDB",
    UtaiteDb: "UtaiteDB"
  },

  notification: {
    search: "検索",
    title: "タイトル",
    artist: "アーティスト",
    releaseDate: "公開日",
    type: {
      song: "曲",
      album: "アルバム",
      artist: "アーティスト",
      report: "報告",
      event: "イベント",
      unknown: "その他"
    },
    header: {
      type: "種類",
      songType: "曲の種類",
      title: "タイトル",
      artist: "アーティスト",
      tags: "タグ",
      releaseDate: "公開日",
      subject: "件名",
      text: "本文"
    }
  },

  buttons: {
    itemsPerPage: "表示件数",
    theme: "テーマ切り替え"
  },

  preferredLanguage: {
    Default: "原語",
    Romaji: "ローマ字",
    English: "英語",
    Japanese: "非英語"
  }
};
