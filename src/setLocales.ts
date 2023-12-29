import i18next from "i18next";

import ar from "../src-tauri/locales/ar.json";
import cs from "../src-tauri/locales/cs.json";
import de from "../src-tauri/locales/de.json";
import en from "../src-tauri/locales/en.json";
import es from "../src-tauri/locales/es.json";
import fr from "../src-tauri/locales/fr.json";
import hu from "../src-tauri/locales/hu.json";
import it from "../src-tauri/locales/it.json";
import ja from "../src-tauri/locales/ja.json";
import pl from "../src-tauri/locales/pl.json";
import pt from "../src-tauri/locales/pt.json";
import ru from "../src-tauri/locales/ru.json";
import tr from "../src-tauri/locales/tr.json";
import zh_CN from "../src-tauri/locales/zh_CN.json";
import zh_TW from "../src-tauri/locales/zh_TW.json";

export const setLocales = (locale: string) => {
  i18next.init({
    lng: locale,
    fallbackLng: "en",
    resources: {
      // https://source.chromium.org/chromium/chromium/src/+/main:ui/base/l10n/l10n_util.cc
      ar: { translation: ar },
      cs: { translation: cs },
      de: { translation: de },
      "de-AT": { translation: de },
      "de-CH": { translation: de },
      "de-DE": { translation: de },
      "de-LI": { translation: de },
      en: { translation: en },
      "en-AU": { translation: en },
      "en-CA": { translation: en },
      "en-GB": { translation: en },
      "en-IE": { translation: en },
      "en-IN": { translation: en },
      "en-NZ": { translation: en },
      "en-US": { translation: en },
      "en-ZA": { translation: en },
      es: { translation: es },
      "es-419": { translation: es },
      "es-AR": { translation: es },
      "es-CL": { translation: es },
      "es-CO": { translation: es },
      "es-CR": { translation: es },
      "es-ES": { translation: es },
      "es-HN": { translation: es },
      "es-MX": { translation: es },
      "es-PE": { translation: es },
      "es-US": { translation: es },
      "es-UY": { translation: es },
      "es-VE": { translation: es },
      fr: { translation: fr },
      "fr-CA": { translation: fr },
      "fr-CH": { translation: fr },
      "fr-FR": { translation: fr },
      hu: { translation: hu },
      it: { translation: it },
      "it-CH": { translation: it },
      "it-IT": { translation: it },
      ja: { translation: ja },
      pl: { translation: pl },
      pt: { translation: pt },
      "pt-BR": { translation: pt },
      "pt-PT": { translation: pt },
      ru: { translation: ru },
      tr: { translation: tr },
      zh: { translation: zh_CN },
      "zh-CN": { translation: zh_CN },
      "zh-TW": { translation: zh_TW },
    },
  });
};
