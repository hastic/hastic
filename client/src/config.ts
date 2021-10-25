export const API_URL = process.env.VUE_APP_API_URL

if(API_URL === undefined) {
  throw new Error("API_URL is undefined!");
}
