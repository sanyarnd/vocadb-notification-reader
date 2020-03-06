import { ru } from "vuetify/src/locale";
export default {
  ...ru,

  connectionError: "Ошибка соединения",
  unexpectedError: "Непредвиденная ошибка",

  delete: "Удалить",
  logout: "Выйти",

  login: {
    username: "Имя пользователя",
    password: "Пароль",
    usernameRequired: "Имя пользователя обязательно",
    passwordRequired: "Пароль обязателен",
    loginVocaDB: "Войти через VocaDB",

    badCredentials: "Неправильные имя пользователя или пароль"
  },

  notification: {
    search: "Поиск",
    type: {
      song: "Песня",
      album: "Альбом",
      artist: "Артист",
      report: "Уведомление",
      event: "Событие",
      unknown: "Неизвестно"
    }
  },

  preferredLanguage: {
    original: "Исходный",
    romanized: "Транслит",
    english: "Английский",
    nonEnglish: "Не английский"
  }
};
