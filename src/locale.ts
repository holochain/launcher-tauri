import { createI18n } from "vue-i18n";

const messages = {
  en: {
    setup: {
      setup: {
        welcome: "Holochain Launcher",
        slogan: "Discover, install and easily manage your Holochain apps",
      },
      login: {
        enterPassword: "Enter password",
        forgotPassword: "forgot password",
        invalidPassword: "Invalid Password.",
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
    appStore: {
      appStore: "App Library",
      howToPublishAnApp: "How to publish an app",
      selectAppFromFileSystem: "Select app from Filesystem",
      noAppsInStore:
        "There are no apps available yet in the App Library or Synchronization with Peers is not complete.",
      readThis: "Read this",
      readThisToPublish:
        " to learn how to publish a Holochain application to the App Library.",
    },
    buttons: {
      ok: "Ok",
      cancel: "Cancel",
      yes: "Yes",
      no: "No",
      continue: "continue",
      next: "Next",
      back: "Back",
      restart: "Restart",
      quit: "Quit",
      getStarted: "Get started",
      letsGo: "Let's Go!",
    },
    dialogs: {
      restart:
        "Do you want to clean up all holochain processes and restart the Holochain Launcher?",
      setupPassword: {
        title: "Set up password",
        part1:
          "To get started, you need to set up a password for the Holochain Launcher.",
        part2:
          "This password is used to secure your cryptographic private keys associated with your identities in Holochain apps.",
        choosePassword: "Choose your password",
        inputPlaceholder: "Choose password",
        repeatPassword: "Repeat your password",
        inputPlaceholder2: "Repeat password",
        passwordsDontMatch: "Passwords don't match.",
        warningPart1: "This password ",
        warningBold1: "cannot be reset or changed ",
        warningPart2:
          "without losing access to your Holochain apps on this device.",
        warningPart3:
          "Make sure to write it on paper right now and create at least one backup of it in a safe place.",
        confirmation:
          "I confirm that I have created a safe backup of my password.",
      },
      forgotPassword: {
        title: "Oh my!",
        part1:
          "Unfortunately, the Holochain Launcher does not support a password recovery mechanism at this stage.",
        part2:
          "Your only option is to do a factory reset (Settings > Factory Reset) to delete your current profile alongside with all your holochain apps and restart from scratch.",
      },
      factoryReset: {
        title: "Factory Reset",
        part1: "This will ",
        bold1: "uninstall all Holochain apps ",
        part2:
          "on your computer of Holochain versions supported by this version of the Holochain Launcher.",
        part3: "It will also ",
        bold2: "delete all data stored in those apps.",
        primaryButton: "Execute Factory Reset",
        executing: "Executing...",
      },
    },
  },
  de: {
    setup: {
      setup: {
        welcome: "Holochain Launcher",
        slogan: "Entdecke, installiere und verwalte deine Holochain Apps",
      },
      login: {
        enterPassword: "Passwort",
        forgotPassword: "Passwort vergessen",
        invalidPassword: "Ungültiges Passwort.",
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
    appStore: {
      appStore: "App Store",
      howToPublishAnApp: "Eigene App Publizieren",
      selectAppFromFileSystem: "Installiere App aus Datei",
      noAppsInStore:
        "Es sind noch keine Apps im App Store verfügbar oder die Synchronisierung mit Peers ist noch unvollständig.",
      readThis: "Lese dies",
      readThisToPublish:
        " um zu lernen, wie man eine Holochain App publiziert.",
    },
    buttons: {
      ok: "Ok",
      cancel: "Abbrechen",
      yes: "Ja",
      no: "Nein",
      continue: "Weiter",
      next: "Weiter",
      back: "Zurück",
      restart: "Neustart",
      quit: "Beenden",
      getStarted: "Loslegen",
      letsGo: "Let's Go!",
    },
    dialogs: {
      restart:
        "Möchtest du alle Holochain Prozesse beenden und den Holochain Launcher neu starten?",
      setupPassword: {
        title: "Passwort erstellen",
        part1:
          "Um loszulegen brauchst du ein Passwort für den Holochain Launcher.",
        part2:
          "Das Passwort wird verwendet, um deine kryptographischen Schlüssel zu schützen, die mit deinen Identitäten in Holochain Apps assoziiert sind.",
        choosePassword: "Wähle dein Passwort",
        inputPlaceholder: "Passwort eingeben",
        repeatPassword: "Wiederhole dein Passwort",
        inputPlaceholder2: "Passwort eingeben",
        passwordsDontMatch: "Passwörter stimmen nicht überein.",
        warningPart1: "Dieses Passwort ",
        warningBold1: "kann nicht zurückgesetzt oder geändert werden ",
        warningPart2:
          "ohne Zugrif zu deinen Holochain Apps auf diesem Gerät zu verlieren.",
        warningPart3:
          "Notiere es jetzt direkt auf Papier und erstelle mindestens ein sicheres Backup an einem sicheren Ort.",
        confirmation:
          "I bestätige, dass ich ein sicheres Backup meines Passworts gemacht habe.",
      },
      forgotPassword: {
        title: "Oh nein!",
        part1:
          "Leider unterstützt Holochain Launcher momentan noch keinen Passwort Wiederherstellungs-Mechanismus.",
        part2:
          "Die einzige Option ist, den Holochain Launcher komplett zurückzusetzen (Settings > Factory Reset) um dein Profil und all deine Holochain Apps zu löschen und von vorne zu beginnen.",
      },
      factoryReset: {
        title: "Zurücksetzen",
        part1: "Dies ",
        bold1: "deinstalliert all deine Holochain Apps ",
        part2:
          "auf deinem Computer von Holochain Versionen die von dieser Version des Holochain Launchers unterstützt werden.",
        part3: "Dabei werden auch ",
        bold2: "alle Daten gelöscht, die in diesen Apps gespeichert sind.",
        primaryButton: "Zurücksetzen",
        executing: "Ausführen...",
      },
    },
  },
};

export const i18n = createI18n({
  locale: "en",
  fallbackLocale: "en",
  messages,
});
