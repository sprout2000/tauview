import i18next from 'i18next';

import en from '../src-tauri/locales/en.json';
import ja from '../src-tauri/locales/ja.json';
import zh_TW from '../src-tauri/locales/zh_TW.json';
/** Merge the pull request sent by PetrTodorov. */
/** https://github.com/sprout2000/leafview/pull/68 */
import cs from '../src-tauri/locales/cs.json';
/** Merge the pull request sent by DrDeee. */
/** https://github.com/sprout2000/leafview/pull/166 */
import de from '../src-tauri/locales/de.json';
/** Merge the pull request sent by singuerinc */
/** https://github.com/sprout2000/leafview/pull/178 */
import es from '../src-tauri/locales/es.json';
/** Merge the pull request sent by nukeop */
/** https://github.com/sprout2000/leafview/pull/214 */
import pl from '../src-tauri/locales/pl.json';
/** Merge the pull request sent by kitt3911 */
/** https://github.com/sprout2000/leafview/pull/215 */
import ru from '../src-tauri/locales/ru.json';
/** Merge the pull request sent by guaycuru */
/** https://github.com/sprout2000/leafview/pull/232 */
import pt from '../src-tauri/locales/pt.json';
/** Merge the pull request sent by ArcherGu */
/** https://github.com/sprout2000/leafview/pull/235 */
import zh_CN from '../src-tauri/locales/zh_CN.json';
/** Merge the pull request sent by SuhaibAtef */
/** https://github.com/sprout2000/leafview/pull/274 */
import ar from '../src-tauri/locales/ar.json';
/** Merge the pull request sent by Levminer */
/** https://github.com/sprout2000/leafview/pull/305 */
import hu from '../src-tauri/locales/hu.json';
/** Merge the pull request sent by BackSpace54 */
/** https://github.com/sprout2000/leafview/pull/312 */
import fr from '../src-tauri/locales/fr.json';

export const setLocales = (locale: string) => {
  i18next.init({
    lng: locale,
    fallbackLng: 'en',
    resources: {
      en: { translation: en },
      ja: { translation: ja },
      cs: { translation: cs },
      de: { translation: de },
      'de-AT': { translation: de },
      'de-CH': { translation: de },
      'de-DE': { translation: de },
      es: { translation: es },
      'es-419': { translation: es },
      pl: { translation: pl },
      ru: { translation: ru },
      pt: { translation: pt },
      'pt-PT': { translation: pt },
      'pt-BR': { translation: pt },
      zh: { translation: zh_CN },
      'zh-CN': { translation: zh_CN },
      'zh-TW': { translation: zh_TW },
      ar: { translation: ar },
      hu: { translation: hu },
      fr: { translation: fr },
    },
  });
};
