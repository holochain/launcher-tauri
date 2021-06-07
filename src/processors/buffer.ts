export function arrayBufferToBase64(buffer: ArrayBuffer): Promise<string> {
  return new Promise((resolve, reject) => {
    const blob = new Blob([buffer], {
      type: "application/octet-binary",
    });
    const reader = new FileReader();
    reader.onload = function (evt) {
      if (
        !evt.target ||
        !evt.target.result ||
        typeof evt.target.result !== "string"
      ) {
        reject("Failed to convert byte array");
        return;
      }
      const dataurl = evt.target.result;
      resolve(dataurl.substr(dataurl.indexOf(",") + 1));
    };
    reader.readAsDataURL(blob);
  });
}
