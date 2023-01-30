import { browser } from "$app/env";
import type { Breaks } from "../generated/Breaks";
import { readable, writable } from "svelte/store";


export function checkUsernameAndPasswordSetInStorage(): boolean {
    // i hate javascript
    return !!(browser && localStorage.getItem(USERNAME_KEY) &&
        localStorage.getItem(PASSWORD_KEY));
}

export const breaks = writable<Breaks>({
    ordered_breaks: [/* {
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
                    }, index: null, notes: "hello", quantity: 2n
                }
            ],
            "customField": {
                "value": "Please call when outside",
                "title": "Notes for delivery",
            },
            buyerNote: "this is a very important note",
        }, twitch_username: "argablarga", order_id: 10019
    }, {
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
                    }, index: null, notes: "hello", quantity: 2n
                }
            ],
            "customField": {
                "value": "Please call when outside",
                "title": "Notes for delivery",
            },
            buyerNote: "this is a very important note",
        }, twitch_username: "argablarga", order_id: 10019
    } */]
})

breaks.subscribe(break_ => {
    console.log(break_);
})

const USERNAME_KEY = 'USERNAME';
const PASSWORD_KEY = 'PASSWORD';

const storedUsername = (browser && localStorage.getItem(USERNAME_KEY)) || '';
const storedPassword = (browser && localStorage.getItem(PASSWORD_KEY)) || '';

export const username = writable(storedUsername);
export const password = writable(storedPassword);
export const baseUrl = readable("http://127.0.0.1:3000");

username.subscribe((newUsername) => {
    browser && localStorage.setItem(USERNAME_KEY, newUsername);
});
password.subscribe((newPassword) => {
    browser && localStorage.setItem(PASSWORD_KEY, newPassword);
});