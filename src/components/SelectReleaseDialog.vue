<template>
  <HCDialog ref="dialog" @closing="$emit('closing-dialog')" closeOnSideClick>

    <div
      class="column"
      style="align-items: center; margin: 10px 15px; padding: 10px 20px"
    >

      <div class="column" style="width: 100%; align-items: flex-start; margin: 12px 0 40px 0;">

        <div class="row" style="align-items: center; margin-bottom: 20px;">
          <!-- if icon provided -->
          <img
            v-if="imgSrc"
            :src="imgSrc"
            alt="app icon"
            style="
              width: 120px;
              height: 120px;
              border-radius: 12px;
              object-fit: cover;
            "
          />
          <!-- if no icon provided -->
          <div
            v-else
            class="column center-content"
            style="
              width: 120px;
              height: 120px;
              border-radius: 12px;
              background: #372ba5;
              margin: 15px;
            "
          >
            <div style="color: white; font-size: 55px; font-weight: 600">
              {{ app.title.slice(0, 2) }}
            </div>
          </div>
          <div
            style="
              font-size: 30px;
              font-weight: 600;
              margin: 0 0 0 30px;
              line-height: 115%;
              word-break: normal;
            "
            :title="app.title"
          >
            {{ app.title }}
          </div>
        </div>

        <div style="margin-left: 10px;"><span style="font-weight: 600;">published by: </span>{{ publisher? `${publisher.name}, ${publisher.location.country}` : "unknown" }}</div>
        <div style="margin-left: 10px;"><span style="font-weight: 600;">first published: </span>{{ (new Date(app.published_at)).toLocaleDateString(locale) }}</div>

        <div style="font-weight: 600; font-size: 20px; margin: 20px 0 10px 0">
          Description:
        </div>

        <div style="max-height: 200px; min-width: 610px; overflow-y: auto; background: #f6f6fa; padding: 5px 10px; border-radius: 10px;">
          {{ app.description }} asjlkd falsdkjf alskjdf alöskdjf alösdkjf aölsdkjf alöskdfj alöskdjf alöskdfj alöskdjf
          alsdjkf alsdkj falskdfj alösdkjf alösdkjf alösdkjf alöskjdf alöskjfdwepoirjf opeirjv eori vjoeirj v oaisjd fasi jfdal
           alsdjf alsfjd alöskfjd aölskjf aölskjfd alösk jfd
           a sdlfjkasöldfkj alsökf jalöskdjf aöefwifj apoijva poijv j alsjdf alskjfd aölskfdj aölskfjd
           pwefj peorifj oeirj voeirj vosivj
           vsj vpsoijv oeirjv oeirjv soiejrv sijv seivj ij jfd
           a sdlfjkasöldfkj alsökf jalöskdjf aöefwifj apoijva poijv j alsjdf alskjfd aölskfdj aölskfjd
           pwefj peorifj oeirj voeirj vosivj
           vsj vpsoijv oeirjv oeirjv soiejrv sijv seivj ijoijv j alsjdf alskjfd aölskfdj aölskfjd
           pwefj peorifj oeirj voeirj vosivj
           vsj vpsoijv oeirjv oeirjv soiejrv sijv seivj ij jfd
           a sdlfjkasöldfkj alsökf jalöskdjf aöefwifj apoijva poijv j alsjdf alskjfd aölskfdj aölskfjd
           pwefj peorifj oeirj voeirj vosivj
        </div>

        <div style="font-weight: 600; font-size: 20px; margin: 30px 0 10px 0">
          Available Releases:
        </div>

        <div v-if="releaseInfos && releaseInfos.length > 0">
          <div class="column card">
            <div style="text-align: right; font-weight: 600">latest Release</div>

            <div class="row" style="align-items: flex-end;">
              <div class="column">
                <div><span style="font-weight: 600;">hApp version:</span> {{ releaseInfos[0].happRelease.content.version }}</div>
                <div><span style="font-weight: 600;">UI version: </span>{{ releaseInfos[0].guiRelease? releaseInfos[0].guiRelease.content.version : "no official UI" }}</div>
                <!-- GUI version as well here? Needs to be fetched independently from a DevHub host -->
              </div>
              <span style="display: flex; flex: 1;"></span>
              <HCButton @click="() => {
                $emit('release-selected', releaseInfos ? releaseInfos[0] : undefined);
                close();
              }">Install</HCButton>
            </div>
          </div>


          <div class="row" style="margin: 20px 0 10px 10px;">
            <div
              @click="showAdvanced = !showAdvanced"
              @keydown.enter="showAdvanced = !showAdvanced"
              class="row advanced-button"
              style="align-items: center;"
              tabindex="0"
            >
              <div
                style="
                  text-align: center;
                  font-size: 24px;
                  width: 20px;
                  cursor: pointer;
                "
              >
                {{ showAdvanced ? "-" : "+" }}
              </div>
              <div style="font-size: 18px; margin-left: 10px">show older releases</div>
            </div>
            <span style="display: flex; flex: 1;"></span>
          </div>

          <div v-show="showAdvanced" class="column" style="align-items: center; width: 100%;">
            <div v-if="releaseInfos.length < 2" style="text-align: center;">
              No older releases available for this app.
            </div>
            <div v-else>
              <div
                v-for="(releaseInfo) of releaseInfos.slice(1)"
                :key="releaseInfo.happRelease.content.version"
                class="row card"
                style="align-items: flex-end; padding-top: 15px;"
              >
                <div class="column">
                  <div><span style="font-weight: 600;">hApp version:</span> {{ releaseInfo.happRelease.content.version }}</div>
                  <div><span style="font-weight: 600;">UI version: </span>{{ releaseInfo.guiRelease? releaseInfo.guiRelease.content.version : "no official UI" }}</div>
                  <!-- GUI version as well here? Needs to be fetched independently from a DevHub host -->
                </div>
                <span style="display: flex; flex: 1;"></span>
                <HCButton @click="() => {
                  $emit('release-selected', releaseInfos ? releaseInfos[0] : undefined);
                  close();
                }">Install</HCButton>
              </div>
            </div>

          </div>
        </div>

        <div v-else-if="releaseInfos && releaseInfos.length < 1" style="text-align: center;">
          There are no installable releases available for this app.
        </div>

        <div v-else-if="getReleaseInfosError" style="background: #f4b2b2; padding: 5px 10px; border-radius: 5px; width: 610px;">
          <b>Error getting releases from peer host:</b><br>
          {{ getReleaseInfosError }}
        </div>

        <div v-else class="column" style="align-items: center; width: 100%;">
          <mwc-circular-progress
            indeterminate
            style="margin-top: 30px; margin-bottom: 20px"
          ></mwc-circular-progress>
        </div>

      </div>


      <HCButton
        style="width: 80px; height: 30px; margin: 4px 6px; margin-bottom: 15px;"
        outlined
        @click="cancel"
      >Close
      </HCButton>

    </div>
  </HCDialog>

  <HCSnackbar
    :timeoutMs="10000"
    :labelText="snackbarText"
    ref="snackbar"
  ></HCSnackbar>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

import HCButton from "./subcomponents/HCButton.vue";
import HCDialog from "./subcomponents/HCDialog.vue";
import HCSnackbar from "./subcomponents/HCSnackbar.vue";

import { ReleaseInfo } from "../types";
import { AppEntry, Entity, HappReleaseEntry, PublisherEntry } from "../appstore/types";
import { AppWebsocket } from "@holochain/client";
import { APPSTORE_APP_ID } from "../constants";
import { collectBytes, fetchGuiReleaseEntry, getHappReleases, getPublisher } from "../appstore/appstore-interface";

export default defineComponent({
  name: "SelectReleaseDialog",
  emits: ["error", "closing-dialog", "release-selected", "cancel"],
  components: {
    HCDialog,
    HCButton,
    HCSnackbar,
  },
  props: {
    app: {
      type: Object as PropType<AppEntry>,
      required: true,
    },
    appWebsocket: {
      type: Object as PropType<any>,
      required: true,
    },
    imgSrc: {
      type: String,
    },
  },
  data(): {
    showAdvanced: boolean;
    snackbarText: string | undefined;
    error: boolean;
    publisher: PublisherEntry | undefined;
    locale: string;
    releaseInfos: Array<ReleaseInfo> | undefined;
    getReleaseInfosError: string | undefined;
  } {
    return {
      showAdvanced: false,
      snackbarText: undefined,
      error: false,
      publisher: undefined,
      locale: "en-US",
      releaseInfos: undefined,
      getReleaseInfosError: undefined,
    };
  },
  mounted () {
    const customLocale = window.localStorage.getItem("customLocale");
    this.locale =  customLocale ? customLocale : navigator.language;
  },
  methods: {
    async getReleaseInfos(): Promise<void> {

      const appStoreInfo = await this.appWebsocket!.appInfo({
        installed_app_id: APPSTORE_APP_ID,
      });

      let happReleases: Array<Entity<HappReleaseEntry>> | undefined = undefined;

      try {
        happReleases = await getHappReleases(this.appWebsocket as AppWebsocket, appStoreInfo, this.app.devhub_address.happ)
      } catch (e) {
        this.getReleaseInfosError = `Failed to find available releases: ${JSON.stringify(e)}`;
        console.error(`Failed to find available releases: ${JSON.stringify(e)}`)
        return;
      }

      if (!happReleases) {
        this.getReleaseInfosError = `Failed to find available releases: happReleases undefined.`;
        return;
      }

      // this.selectedHappReleases = happReleases.map((entity) => entity.content).sort((a, b) => b.last_updated - a.last_updated);
      let releaseInfos: Array<ReleaseInfo> = [];

      console.log("@getReleaseInfos: fetching gui release entries...");

      try {
        await Promise.all(happReleases.map(
          async (happReleaseEntity) => {
            let releaseInfo: ReleaseInfo = {
              happRelease: happReleaseEntity,
              guiRelease: undefined,
            };

            const guiReleaseHash = happReleaseEntity.content.official_gui;
            if (guiReleaseHash) {
              const guiReleaseEntry = await fetchGuiReleaseEntry(this.appWebsocket as AppWebsocket, appStoreInfo, guiReleaseHash);
              releaseInfo.guiRelease = guiReleaseEntry;
            }

            releaseInfos.push(releaseInfo);
          })
        );
      } catch (e) {
        if (JSON.stringify(e).includes("No available peer host found")) {
          this.getReleaseInfosError = "No available peer host found.";
          return;
        } else {
          this.getReleaseInfosError = `Failed to find available releases: ${JSON.stringify(e)}`;
          console.error(`Failed to find available releases: ${JSON.stringify(e)}`)
          return;
        }
      }

      this.releaseInfos = releaseInfos.sort((a, b) => b.happRelease.content.published_at - a.happRelease.content.published_at);
      this.getReleaseInfosError = undefined;
    },
    async open() {
      (this.$refs.dialog as typeof HCDialog).open();

      let appStoreInfo = undefined;

      try {
        appStoreInfo = await this.appWebsocket!.appInfo({
          installed_app_id: APPSTORE_APP_ID,
        });
        console.log("@mounted: getting publisher...");
        this.publisher = await getPublisher(this.appWebsocket, appStoreInfo, this.app.publisher);
        console.log("@mounted: got publisher: ", this.publisher);
      } catch (e) {
        console.error(`Failed to get publisher info: ${JSON.stringify(e)}`)
      }

      await this.getReleaseInfos();

    },
    close() {
      (this.$refs.dialog as typeof HCDialog).close();
      this.$emit('closing-dialog');
    },
    cancel() {
      (this.$refs.dialog as typeof HCDialog).close();
      this.$emit('cancel');
    }
  },
});
</script>

<style scoped>

.advanced-button:hover {
  opacity: 0.8;
  cursor: pointer;
}


.card {
  min-width: 600px;
  background: #f6f6fa;
  border-radius: 15px;
  box-shadow: 0 0px 5px #9b9b9b;
  /* border: 2px solid #e1e1e1; */
  padding: 5px 15px 15px 15px;
  margin: 12px 0;
}
</style>
