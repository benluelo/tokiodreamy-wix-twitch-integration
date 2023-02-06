import { browser } from "$app/environment";
import type { Breaks } from "../generated/Breaks";
import type { OrderNumber } from "../generated/OrderNumber";
import { get, readable, writable } from "svelte/store";
import type { SseEvent } from "../generated/SseEvent";
import { baseUrl, breaks, password, username } from "./stores";
import type { OrderUpdate } from "../generated/OrderUpdate";


export async function login() {
    if (!browser) {
        throw new Error("not in browser context");
    }

    if (get(loginStatus) === LoginStatus.Success) {
        return
    }

    let auth_header = authHeader();
    let resp = await fetch(`${get(baseUrl)}/login`, {
        headers: {
            Authorization: auth_header,
        },
    });

    console.log("resp.status", resp.status);


    if (resp.status === 200) {
        loginStatus.set(LoginStatus.Success);
    } else {
        loginStatus.set(LoginStatus.Error);
    }
}

function authHeader() {
    return window.btoa(`${get(username)}:${get(password)}`);
}

export async function orderCompleted(orderNumber: OrderNumber) {
    return await fetch(`${get(baseUrl)}/order_completed/${orderNumber}`, {
        headers: {
            Authorization: authHeader(),
        },
        method: "POST"
    }).then(() => {
        
    })
}

export async function updateOrder(orderNumber: OrderNumber, orderUpdate: OrderUpdate) {
    return await fetch(`${get(baseUrl)}/update_order/${orderNumber}`, {
        headers: {
            Authorization: authHeader(),
            "Content-Type": "application/json",
        },
        method: "POST", body: JSON.stringify(orderUpdate)
    }).then(() => {
        
    })
}

export async function registerSse(): Promise<void> {
    const source = new EventSource(`${get(baseUrl)}/sse`);

    source.onmessage = (msg: MessageEvent<string>) => {
        console.log(msg);

        const parsedJson = JSON.parse(msg.data);
        if (parsedJson.BreaksUpdated) {
            breaks.set(parsedJson.BreaksUpdated)
        }
    }

    eventSource.set(source);
}

const eventSource = writable<EventSource | undefined>();

export const loginStatus = writable<LoginStatus | undefined>(undefined);

export enum LoginStatus {
    Success,
    Error,
}