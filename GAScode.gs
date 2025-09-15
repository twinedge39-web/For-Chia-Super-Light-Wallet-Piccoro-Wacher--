// === Config ===
const SHARED_TOKEN = "your-long-secret-token"; // ←必ずランダム長に
const FIXED_TO     = "your@gmail.com";         // ←宛先を固定（中継悪用防止）
const FROM_NAME    = "Wallet Piccoro";         // 表示名
const MAX_SUBJ     = 120;                      // 迷惑対策
const MAX_BODY     = 4000;                     // GAS/Gmail制限も考慮

function doPost(e) {
  // 1) POST以外は拒否
  if (!e || !e.postData) {
    return _json({ ok:false, error:"method_not_allowed" }, 405);
  }

  // 2) JSON優先でパース、ダメならフォーム
  let data = {};
  try {
    if (e.postData.type && e.postData.type.indexOf("application/json") !== -1) {
      data = JSON.parse(e.postData.contents || "{}");
    } else {
      data = e.parameter || {};
    }
  } catch (err) {
    return _json({ ok:false, error:"bad_json", detail: String(err) }, 400);
  }

  // 3) トークン検証
  if (!data.token || data.token !== SHARED_TOKEN) {
    return _json({ ok:false, error:"unauthorized" }, 401);
  }

  // 4) 宛先は固定（受け取っても無視）
  const to = FIXED_TO;

  // 5) 入力をサニタイズ/ガード
  let subject = String(data.subject || "Wallet Piccoro Notification");
  let body    = String(data.body    || "No body");
  if (subject.length > MAX_SUBJ) subject = subject.slice(0, MAX_SUBJ) + "…";
  if (body.length    > MAX_BODY) body    = body.slice(0, MAX_BODY)   + "\n…(truncated)";

  // 6) 送信
  GmailApp.sendEmail(to, subject, body, { name: FROM_NAME });

  // 7) 成功レスポンス（トークンは絶対に出さない）
  return _json({ ok:true });
}

// 共通: JSONレスポンス
function _json(obj, status) {
  const out = ContentService.createTextOutput(JSON.stringify(obj))
    .setMimeType(ContentService.MimeType.JSON);
  if (status) {
    // Apps Scriptはステータス直接変更できないが、クライアント側はbodyで判定可能
  }
  return out;
}
