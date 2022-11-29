import { createI18n } from "vue-i18n";

const messages = {
  en: {
    setup: {
      setup: {},
      login: {
        enterPassword: "Enter password",
        forgotPassword: "forgot password",
        slogan: "Discover, install and easily manage your Holochain apps",
      },
    },
    main: {
      installedApps: "Installed Apps",
      webApps: "Web Apps",
      webAppsHelper: "Holochain Apps with Graphical User Interface",
      headlessApps: "Headless Apps",
      headlessAppsHelper: "Holochain Apps without Graphical User Interface",
      holochainVersions: "Holochain Versions",
      holochainVersionsHelper: "Installed Holochain Versions",
      noHeadlessApps: "There are no headless apps installed",
      noWebApps: "There are no web apps installed",
      noHolochainVersions: "There are no Holochain versions installed.",
      inThisVersion: "in this Holochain Version.",
      sortBy: "sort by",
      refresh: "Refresh",
      installNewApp: "INSTALL NEW APP",
      reportIssue: "Report Issue",
      hide: "Hide",
      show: "Show",
    },
    appLibrary: {
      appLibrary: "App Library",
      howToPublishAnApp: "How to publish an app",
      selectAppFromFileSystem: "Select app from Filesystem",
      noAppsInDevHub:
        "There are no apps available yet in the DevHub or Synchronization with Peers is not complete.",
      readThisToPublish:
        "Read this to learn how to publish a Holochain application to the DevHub.",
    },
    buttons: {
      continue: "continue",
      ok: "Ok",
      cancel: "Cancel",
      next: "Next",
    },
  },
  de: {
    setup: {
      setup: {},
      login: {
        enterPassword: "Passwort",
        forgotPassword: "Passwort vergessen",
        slogan: "Entdecke, installiere und verwalte deine Holochain Apps",
      },
    },
    main: {
      installedApps: "Installierte Apps",
      webApps: "Web Apps",
      webAppsHelper: "Holochain Apps mit Graphischer Benutzeroberfläche",
      headlessApps: "Headless Apps",
      headlessAppsHelper: "Holochain Apps ohne Graphische Benutzeroberfläche",
      holochainVersions: "Holochain Versionen",
      holochainVersionsHelper: "Installierte Holochain Versionen",
      noHeadlessApps: "Es sind keine Headless Apps installiert",
      noWebApps: "Es sind keine Web Apps installiert",
      noHolochainVersions: "Es sind keine Holochain Versionen installiert.",
      inThisVersion: "in dieser Holochain Version.",
      sortBy: "sortiere nach",
      refresh: "Aktualisieren",
      installNewApp: "APP INSTALLIEREN",
      reportIssue: "Problem Melden",
      hide: "Verbergen",
      show: "Zeigen",
    },
    appLibrary: {
      appLibrary: "App Store",
      howToPublishAnApp: "Eigene App Publizieren",
      selectAppFromFileSystem: "Installiere App aus Datei",
      noAppsInDevHub:
        "Es sind noch keine Apps im DevHub verfügbar oder die Synchronisierung mit Peers is noch unvollständig.",
      readThisToPublish:
        "Lese dies um zu lernen, wie man eine Holochain App publiziert.",
    },
    buttons: {
      continue: "Weiter",
      ok: "Ok",
      cancel: "Abbrechen",
      next: "Weiter",
    },
  },
};

export const i18n = createI18n({
  locale: "de",
  fallbackLocale: "en",
  messages,
});
