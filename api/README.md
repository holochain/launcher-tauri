# @holochain/launcher-api

This package contains the API for Holochain apps to interact with the Holochain Launcher for things like sending notifications to the operating system.


## Example Usage

```typescript
import { notifyLauncher, launcherApiAvailable } from "@holochain/launcher-api";

// In case your happ UI supports multiple environments, you can check whether the launcher API is available in the first place
if (launcherApiAvailable) {
  const notification = {
    title: "Test";
    body: "This is a test notification to the Holochain Launcher";
    notification_type: "test";
    icon_file_name: "test_notification.png";
    urgency: "medium";
    timestamp: someHolochainRecord.signed_action.hashed.content.timestamp;
  }

  await notifyLauncher([notification]);
}

// You may also want to send a notification where you reset the notification count manually

import { resetNotificationCount } from "@holochain/launcher-api";
import { encodeHashToBase64 } from "@holochain/client";

if (launcherApiAvailable) {
  const notificationId = encodeHashToBase64(someHolochainRecord.signed_action.hashed.hash);

  const notification = {
    title: "Test";
    body: "This is another test notification. This time it comes with a notification id and the notification count will need to be reset by the hApp.";
    notification_type: "test";
    icon_file_name: "test_notification.png";
    urgency: "medium";
    timestamp: someHolochainRecord.signed_action.hashed.content.timestamp;
    custom_count_reset: notificationId,
  }

  await notifyLauncher([notification]);

  // Then later when your hApp's user read your notification, you need to reset the notification count in the Launcher:
  await resetNotificationCount([notificationId]);
}

```