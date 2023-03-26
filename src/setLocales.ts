import i18next from "i18next";

import ar from "../src-tauri/locales/ar.json";
import cs from "../src-tauri/locales/cs.json";
import de from "../src-tauri/locales/de.json";
import en from "../src-tauri/locales/en.json";
import es from "../src-tauri/locales/es.json";
import fr from "../src-tauri/locales/fr.json";
import hu from "../src-tauri/locales/hu.json";
import ja from "../src-tauri/locales/ja.json";
import pl from "../src-tauri/locales/pl.json";
import pt from "../src-tauri/locales/pt.json";
import ru from "../src-tauri/locales/ru.json";
import zh_CN from "../src-tauri/locales/zh_cn.json";
import zh_TW from "../src-tauri/locales/zh_tw.json";

export const setLocales = (locale: string) => {
  i18next.init({
    lng: locale,
    fallbackLng: "en",
    resources: {
      en: { translation: en },
      ja: { translation: ja },
      cs: { translation: cs },
      de: { translation: de },
      "de-AT": { translation: de },
      "de-CH": { translation: de },
      "de-DE": { translation: de },
      es: { translation: es },
      "es-419": { translation: es },
      pl: { translation: pl },
      ru: { translation: ru },
      pt: { translation: pt },
      "pt-PT": { translation: pt },
      "pt-BR": { translation: pt },
      zh: { translation: zh_CN },
      "zh-CN": { translation: zh_CN },
      "zh-TW": { translation: zh_TW },
      ar: { translation: ar },
      hu: { translation: hu },
      fr: { translation: fr },
    },
  });
};
