import { browser } from '$app/environment';
import type { Breaks } from '../generated/Breaks';
import { readable, writable } from 'svelte/store';

export function checkUsernameAndPasswordSetInStorage(): boolean {
  // i hate javascript
  return !!(browser && localStorage.getItem(USERNAME_KEY) && localStorage.getItem(PASSWORD_KEY));
}

import { PUBLIC_SERVER_BASE_URL } from '$env/static/public'

export const breaks = writable<Breaks>({
  ordered_breaks: []
});

breaks.subscribe((break_) => {
  console.log(break_);
});

const USERNAME_KEY = 'USERNAME';
const PASSWORD_KEY = 'PASSWORD';

const storedUsername = (browser && localStorage.getItem(USERNAME_KEY)) || '';
const storedPassword = (browser && localStorage.getItem(PASSWORD_KEY)) || '';

export const username = writable(storedUsername);
export const password = writable(storedPassword);
export const serverBaseUrl = readable(PUBLIC_SERVER_BASE_URL);

username.subscribe((newUsername) => {
  browser && localStorage.setItem(USERNAME_KEY, newUsername);
});
password.subscribe((newPassword) => {
  browser && localStorage.setItem(PASSWORD_KEY, newPassword);
});
