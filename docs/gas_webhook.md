# GAS Webhook Script for Wallet Piccoro

Wallet Piccoro uses Google Apps Script (GAS) as a lightweight webhook service to forward balance notifications to Gmail.  
This document contains the complete source code and deployment steps.

---

## Source Code

```javascript
const SHARED_TOKEN = "your-long-secret-token";

function doPost(e) {
  let data = {};
  try {
    if (e.postData && e.postData.type &&
        e.postData.type.indexOf('application/json') !== -1) {
      data = JSON.parse(e.postData.contents || "{}");
    } else {
      data = e.parameter || {};
    }
  } catch (err) {
    return ContentService.createTextOutput(JSON.stringify({ ok:false, error: "bad_json" }))
      .setMimeType(ContentService.MimeType.JSON);
  }

  if (data.token !== SHARED_TOKEN) {
    return ContentService.createTextOutput(JSON.stringify({ ok:false, error:"unauthorized" }))
      .setMimeType(ContentService.MimeType.JSON);
  }

  const to      = data.to      || "your@gmail.com";
  const subject = data.subject || "Wallet Piccoro Notification";
  const body    = data.body    || "No body";

  GmailApp.sendEmail(to, subject, body, { name: "Wallet Piccoro" });

  return ContentService.createTextOutput(JSON.stringify({ ok:true }))
    .setMimeType(ContentService.MimeType.JSON);
}
```

---

## Deployment

1. Open [Google Apps Script](https://script.google.com/)
2. Create a **new project** and paste the code above into `Code.gs`
3. Replace `your-long-secret-token` with a strong random string
4. Save the project
5. Go to **Deploy → New deployment → Web app**
   - Execute as: **Me**
   - Who has access: **Anyone**
6. Deploy → Copy the `/exec` URL



## Config Example
In `config.json`:

```json
"webhook": {
  "url": "https://script.google.com/macros/s/XXXX/exec",
  "token": "your-long-secret-token",
  "to": "your@gmail.com"
}
```

---

## Notes
- The token is mandatory for basic authentication.
- Keep SHARED_TOKEN secret (do not commit it to git).
- Emails are sent via your Google account — quota applies (~100/day for free accounts).
- If you need multiple recipients, extend the GAS code to split data.to by comma.

