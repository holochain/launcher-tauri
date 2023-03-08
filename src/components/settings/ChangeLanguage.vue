<template>
  <HCGenericDialog
    ref="dialog"
    @confirm="saveLanguage()"
    :primaryButtonLabel="$t('buttons.save')"
    :primaryButtonDisabled="!selectedLanguage"
    closeOnSideClick
  >
    <div
      class="column"
      style="min-height: 200px; min-width: 400px; margin: -30px 20px 20px 20px"
    >
      <div class="row" style="justify-content: center">
        <div style="font-weight: 600; font-size: 25px; margin: 20px 0 10px 0">
          {{ $t("dialogs.changeLanguage.languageSettings") }}
        </div>
      </div>
      <div class="row" style="margin-top: 20px; margin-bottom: 16px">
        <HCSelect
          ref="selectLanguage"
          style="margin: 5px; width: 360px"
          :label="$t('setup.changeLanguage')"
          :items="languages"
          @item-selected="selectedLanguage = $event"
        >
        </HCSelect>
      </div>

    </div>
  </HCGenericDialog>
</template>

<script lang="ts">
import { getCurrent } from "@tauri-apps/api/window";
import { defineComponent } from "vue";
import { languageNames, i18n } from "../../locale";

import HCGenericDialog from "../subcomponents/HCGenericDialog.vue";
import HCSelect from "../subcomponents/HCSelect.vue";

export default defineComponent({
  name: "ChangeLanguage",
  components: { HCGenericDialog, HCSelect },
  data(): {
    languages: [string, string][];
    selectedLanguage: string | undefined;
  } {
    return {
      languages: [
        // ["ERROR", "ERROR"],
        // ["WARN (default)", "WARN"],
        // ["INFO", "INFO"],
        // ["DEBUG", "DEBUG"],
        // ["TRACE", "TRACE"],
      ],
      selectedLanguage: undefined,
    };
  },
  mounted() {
    let languages: [string, string][] = [];
    i18n.global.availableLocales.forEach((locale) => {
      if (languageNames[locale]) {
        languages.push([languageNames[locale], locale]);
      } else {
        languages.push([locale, locale]);
      }
    });
    this.languages = languages;

    this.$nextTick(async () => {
      const dialog = this.$refs.dialog as typeof HCGenericDialog;
      await getCurrent().listen("open-language-settings", async () => {
        dialog.open();
      });
    });
  },
  methods: {
    async saveLanguage() {
      if (this.selectedLanguage) {
        window.localStorage.setItem("customLocale", this.selectedLanguage!);
      } else {
        alert("No language selected.");
      }
      window.location.reload();
    },
    open() {
      (this.$refs.dialog as typeof HCGenericDialog).open();
    }
  },
});
</script>
