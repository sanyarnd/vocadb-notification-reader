import type { Messages } from "./en";

export const ru: Messages = {
  connectionError: "Ошибка соединения",
  delete: "Удалить",
  logout: "Выйти",
  toDatabase: "Открыть в {0}",
  noData: "<нет данных>",

  login: {
    username: "Имя пользователя",
    password: "Пароль",
    usernameRequired: "Имя пользователя обязательно",
    passwordRequired: "Пароль обязателен",
    loginWith: "Войти через {0}",
    badCredentials: "Неправильные имя пользователя или пароль"
  },

  database: {
    VocaDb: "VocaDB",
    TouhouDb: "TouhouDB",
    UtaiteDb: "UtaiteDB"
  },

  notification: {
    search: "Поиск",
    title: "Название",
    artist: "Исполнитель",
    releaseDate: "Дата выхода",
    type: {
      song: "Песня",
      album: "Альбом",
      artist: "Артист",
      report: "Жалоба",
      event: "Событие",
      unknown: "Неизвестно"
    },
    header: {
      type: "Тип",
      songType: "Тип песни",
      title: "Название",
      artist: "Исполнитель",
      tags: "Теги",
      releaseDate: "Дата выхода",
      subject: "Тема",
      text: "Текст"
    }
  },

  buttons: {
    itemsPerPage: "Элементов на странице",
    theme: "Сменить тему"
  },

  preferredLanguage: {
    Default: "Исходный",
    Romaji: "Транслит",
    English: "Английский",
    Japanese: "Не английский"
  }
};
