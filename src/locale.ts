import { createI18n } from "vue-i18n";

// a mapping between locale abbreviations and full Language names
export const languageNames = {
  "de": "🇩🇪 Deutsch",
  "en": "🇬🇧 English",
  "fr": "🇫🇷 Français",
  "it": "🇮🇹 Italiano",
}

const messages = {

  // =========================================================================
  // 🇬🇧 ENGLISH
  // =========================================================================

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
      changeLanguage: "change language",
      settingUp: "Setting up",
      startingUp: "Starting up",
    },
    main: {
      activePeerSynchronization: "Active Peer Synchronization",
      allVersions: "All Versions",
      copied: "Copied",
      copyDnaHash: "Copy Dna Hash",
      copyNetworkSeed: "Copy Network Seed",
      cloneName: "Name",
      headlessApps: "Headless Apps",
      headlessAppsHelper: "Holochain Apps without Graphical User Interface",
      hide: "Hide",
      hideNetworkSeed: "Hide Network Seed",
      holochainVersions: "Holochain Versions",
      holochainVersion: "Holochain Version",
      holochainVersionsHelper: "Installed Holochain Versions",
      incoming: "incoming",
      installNewApp: "INSTALL NEW APP",
      inThisVersion: "in this Holochain Version.",
      launcher: "Launcher",
      name: "name",
      nameDescending: "name descending",
      networkSeed: "Network Seed",
      noClonedCells: "There are no cloned cells in this app.",
      noDisabledClonedCells: "There are no disabled cloned cells in this app.",
      noHeadlessApps: "There are no headless apps installed",
      noWebApps: "There are no web apps installed",
      noHolochainVersions: "There are no Holochain versions installed.",
      outgoing: "outgoing",
      refresh: "Refresh",
      reportIssue: "Report Issue",
      settings: "Settings",
      sortBy: "sort by",
      show: "Show",
      showNetworkSeed: "Show Network Seed",
      visiblePeers: "Visible Peers",
      webApps: "Web Apps",
      webAppsHelper: "Holochain Apps with Graphical User Interface",
      yourPublicKey: "Your Public Key",
    },
    appStore: {
      activeDataExchanges: "currently ongoing data exchanges with peers",
      appStore: "App Store",
      appLibrarySynchronization: "App Library Synchronization",
      fullSynchronizationRequired: "Full synchronization with peers required to reliably download all apps.",
      noOngoingPeerSynchronization: "no ongoing peer synchronization",
      selectAppFromFileSystem: "Select app from Filesystem",
      noAppsInStore:
        "There are no apps available yet in the App Library or Synchronization with Peers is not complete.",
      synchronizationNotCompleteError: "App Library Synchronization not Complete. Please try again later."
    },
    settings: {
      publishAnApp: "Publish an app"
    },
    buttons: {
      back: "Back",
      cancel: "Cancel",
      continue: "continue",
      disable: "Disable",
      enable: "Enable",
      getStarted: "Get started",
      install: "Install",
      letsGo: "Let's Go!",
      next: "Next",
      no: "No",
      ok: "Ok",
      quit: "Quit",
      restart: "Restart",
      save: "Save",
      start: "Start",
      uninstall: "Uninstall",
      update: "Update",
      yes: "Yes",
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
        optionalDeletions: "Optional:",
        deleteLair: "Delete Private Keys",
        deleteLogs: "Delete Logs",
        deleteAllHolochainVersions: "(not recommended) Delete data of ALL Holochain versions. This includes Holochain versions that are or were used by other versions of the Holochain Launcher."
      },
      guiUpdate: {
        title: "UI Update",
        mainText: "There's a new UI available for this app",
        changelog: "changelog",
        version: "version",
        publishedAt: "published",
        question: "Would you like to install it?",
      },
      changeLanguage: {
        language: "Language",
        languageSettings: "Language Settings",
      },
      confirmUninstallApp: "Are you sure you want to uninstall this App? This will irrevocably delete all data stored in it.",
      confirmUninstallCell: "Are you sure you want to delete this cell? This will irrevocably delete all data stored in it.",
    },
  },

  // =========================================================================
  // 🇩🇪 GERMAN
  // =========================================================================

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
      changeLanguage: "Sprache ändern",
      startingUp: "Starten",
      settingUp: "Einrichten",
    },
    main: {
      activePeerSynchronization: "Aktive Peer Synchronisationen",
      allVersions: "Alle Versionen",
      copied: "Kopiert",
      copyDnaHash: "Dna Hash kopieren",
      copyNetworkSeed: "Network Seed kopieren",
      cloneName: "Name",
      headlessApps: "Headless Apps",
      headlessAppsHelper: "Holochain Apps ohne Graphische Benutzeroberfläche",
      hide: "Verbergen",
      hideNetworkSeed: "Network Seed verbergen",
      holochainVersions: "Holochain Versionen",
      holochainVersion: "Holochain Version",
      holochainVersionsHelper: "Installierte Holochain Versionen",
      incoming: "eingehend",
      installedApps: "Installierte Apps",
      installNewApp: "APP INSTALLIEREN",
      inThisVersion: "in dieser Holochain Version.",
      name: "Name",
      nameDescending: "Name absteigend",
      networkSeed: "Network Seed",
      noClonedCells: "Diese App hat keine Cloned Cells.",
      noDisabledClonedCells: "Diese App hat keine deaktivierten Cloned Cells.",
      noHeadlessApps: "Es sind keine Headless Apps installiert",
      noWebApps: "Es sind keine Web Apps installiert",
      noHolochainVersions: "Es sind keine Holochain Versionen installiert.",
      outgoing: "ausgehend",
      refresh: "Aktualisieren",
      reportIssue: "Problem Melden",
      show: "Zeigen",
      showNetworkSeed: "Network Seed anzeigen",
      sortBy: "sortiere nach",
      visiblePeers: "Sichtbare Peers",
      webApps: "Web Apps",
      webAppsHelper: "Holochain Apps mit Graphischer Benutzeroberfläche",
      yourPublicKey: "Dein Public Key",
    },
    appStore: {
      activeDataExchanges: "Zurzeit aktiver Datenaustausch mit Peers",
      appStore: "App Store",
      appLibrarySynchronization: "App Store Synchronisation",
      fullSynchronizationRequired: "Vollständige Synchronisation mit Peers erforderlich, um alle Apps herunterladen zu können.",
      noOngoingPeerSynchronization: "keine aktiven Synchronisationen mit Peers",
      selectAppFromFileSystem: "Installiere App von Datei",
      noAppsInStore:
        "Es sind noch keine Apps im App Store verfügbar oder die Synchronisierung mit Peers ist noch unvollständig.",
      synchronizationNotCompleteError: "App Store Synchronisation noch nicht abgeschlossen. Bitte versuche es später erneut."
    },
    settings: {
      publishAnApp: "Eigene App Publizieren",
    },
    buttons: {
      back: "Zurück",
      cancel: "Abbrechen",
      continue: "Weiter",
      disable: "Deaktivieren",
      enable: "Aktivieren",
      getStarted: "Loslegen",
      install: "Installieren",
      letsGo: "Let's Go!",
      next: "Weiter",
      no: "Nein",
      ok: "Ok",
      restart: "Neustart",
      save: "Speichern",
      start: "Starten",
      uninstall: "Deinstallieren",
      update: "Update",
      quit: "Beenden",
      yes: "Ja",
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
        optionalDeletions: "Optional:",
        deleteLair: "Private Schlüssel löschen",
        deleteLogs: "Log Dateien löschen",
        deleteAllHolochainVersions: "(Nicht empfohlen) Daten ALLER Holochain Versionen löschen. Dies beinhaltet Holochain Versionen, die von anderen Versionen des Holochain Launcher verwendet werden oder wurden."
      },
      guiUpdate: {
        title: "UI Update",
        mainText: "Es ist ein neues UI verfügbar für diese App",
        changelog: "Änderungen",
        version: "Version",
        publishedAt: "Publiziert",
        question: "Möchtest du es installieren?",
      },
      changeLanguage: {
        language: "Sprache",
        languageSettings: "Sprach-Einstellungen",
      },
      confirmUninstallApp: "Bist du sicher, dass du diese App deinstallieren möchtest? Dies wird unwiderruflich alle Daten löschen, die darin gespeichert sind.",
      confirmUninstallCell: "Bist du sicher, dass du diese Cell deinstallieren möchtest? Dies wird unwiderruflich alle Daten löschen, die darin gespeichert sind.",
    },
  },
};



export const i18n = createI18n({
  locale: "en",
  fallbackLocale: "en",
  messages,
});
