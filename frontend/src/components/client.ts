import { browser } from "$app/env";
import { goto } from "$app/navigation";
import type { Breaks } from "../generated/Breaks";
import type { OrderNumber } from "../generated/OrderNumber";
import { derived, get, readable, writable } from "svelte/store";
import type { SseEvent } from "src/generated/SseEvent";

export class Client {
    constructor() { }

    // TODO: https://www.npmjs.com/package/esbuild-svelte

    public async login() {
        let auth_header = this.authHeader();
        let resp = await fetch(`${get(baseUrl)}/login`, {
            headers: {
                Authorization: auth_header,
            },
        });

        if (resp.status === 200) {
            return LoginStatus.Success;
        } else {
            return LoginStatus.Error;
        }
    }

    private authHeader() {
        return window.btoa(`${get(username)}:${get(password)}`);
    }

    public async orderCompleted(orderNumber: OrderNumber) {
        return await fetch(`${get(baseUrl)}/order_completed`, {
            headers: {
                Authorization: this.authHeader(),
            },
        })
    }

    public static async sse() {
        const source = new EventSource("http://127.0.0.1:3000/sse");

        source.onmessage = (msg: MessageEvent<SseEvent>) => {
            if (msg.data.BreaksUpdated) {
                breaks.set(msg.data.BreaksUpdated)
            }
        }
    }
}

export const breaks = writable<Breaks>({
    ordered_breaks: [{
        order: {
            "number": 10019,
            "lineItems": [
                {
                    "name": "my product's name",
                    "options": [
                        {
                            "option": "Size",
                            "selection": "Medium"
                        }
                    ],
                    "customTextFields": [
                        {
                            "title": "Notes for delivery",
                            "value": "Please leave at front door"
                        }
                    ],
                    "mediaItem": {
                        "altText": "This is a description of the image",
                        "id": "fac9dc352bf7d54ed0458d64ce41a3ec.jpg",
                        "src": "wix:image://v1/fac9dc352bf7d54ed0458d64ce41a3ec.jpg/file.jpg#originWidth=1348&originHeight=899",
                    }, index: null, notes: null, quantity: 4n
                },
                {
                    "name": "argablaldsfj lksjdlflkkkjsd  lkjsd",
                    "options": [
                        {
                            "option": "Size",
                            "selection": "Medium"
                        },
                        {
                            "option": "Size",
                            "selection": "Medium"
                        },
                        {
                            "option": "Size",
                            "selection": "Medium"
                        }
                    ],
                    "customTextFields": [
                        {
                            "title": "Notes for delivery",
                            "value": "Please leave at front door"
                        }
                    ],
                    "mediaItem": {
                        "altText": "This is a description of the image",
                        "id": "fac9dc352bf7d54ed0458d64ce41a3ec.jpg",
                        "src": "wix:image://v1/fac9dc352bf7d54ed0458d64ce41a3ec.jpg/file.jpg#originWidth=1348&originHeight=899",
                    }, index: null, notes: null, quantity: 10n
                },
                {
                    "name": "my product's name",
                    "options": [
                        {
                            "option": "Size",
                            "selection": "Medium"
                        },
                        {
                            "option": "Size",
                            "selection": "Medium"
                        }
                    ],
                    "customTextFields": [
                        {
                            "title": "sldjfldsjkf sdfk",
                            "value": "ldsfljsdfj"
                        },
                        {
                            "title": "a;lsdf",
                            "value": "0498witekrngflikjsd lsadf"
                        }
                    ],
                    "mediaItem": {
                        "altText": "This is a description of the image",
                        "id": "fac9dc352bf7d54ed0458d64ce41a3ec.jpg",
                        "src": "wix:image://v1/fac9dc352bf7d54ed0458d64ce41a3ec.jpg/file.jpg#originWidth=1348&originHeight=899",
                    }, index: null, notes: null, quantity: 2n
                }
            ],
            "customField": {
                "value": "Please call when outside",
                "title": "Notes for delivery",
            },
            buyerNote: "this is a very important note",
        }, twitch_username: "argablarga", order_id: 10019
    }]
})

const USERNAME_KEY = 'USERNAME';
const PASSWORD_KEY = 'PASSWORD';

const storedUsername = (browser && localStorage.getItem(USERNAME_KEY)) || '';
const storedPassword = (browser && localStorage.getItem(PASSWORD_KEY)) || '';

export const username = writable(storedUsername);
export const password = writable(storedPassword);
export const baseUrl = readable("http://127.0.0.1:3000");

export const client = readable(new Client());

username.subscribe((newUsername) => {
    browser && localStorage.setItem(USERNAME_KEY, newUsername);
});
password.subscribe((newPassword) => {
    browser && localStorage.setItem(PASSWORD_KEY, newPassword);
});

export enum LoginStatus {
    Success,
    Error,
}