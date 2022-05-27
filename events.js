// Place this code in the events.js file
// of your site's Backend section.

const AUTH_KEY = "&76_Gmr&gykSFm*t5r!GwmdA3Lmf4H=65xP?Q_WcTJJJw+W47!&KK&wAyJpAWycA!?AMZzTu%hNJ-MEapj6vc%5d@nS+JFVdM_GC=-%@FNWgexwMNzXk*dtT%=kzJwu@XDy-ksM?wvF_JFV!*PD?_G79h3yYgx=fz3thravn?uhXsH6%yz8Svavm9$vwDfBsybqWeDt!e*v_Dkv^R29KPdw2&Xpc=VZQXX?EEFmBU$q2g#Fau_%y-L6#FqQD%86v";

export function wixStores_onOrderPaid(event) {
  let obj = {
    buyerNote: event.buyerNote,
    number: event.number,
    lineItems: event.lineItems,
    customField: event.customField,
  }

  fetch("http://18.212.208.3:3000/new_order", {
    method: 'POST',
    body: JSON.stringify(obj),
    headers: {
      'Content-Type': 'application/json',
      'Authorization': Buffer.from(`wix:${AUTH_KEY}`).toString('base64')
    },
  })
}
