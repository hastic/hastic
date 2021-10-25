import { User } from "@/types/user";
import { API_URL } from "@/config";

import axios from 'axios';

// TODO: get it from config
const AUTH_API_URL = API_URL + 'auth/';

class AuthService {
  login(user: User) {
    return axios
      .post(AUTH_API_URL + 'signin', {
        username: user.username,
        password: user.password
      })
      .then((response: any) => {
        if (response.data.accessToken) {
          localStorage.setItem('user', JSON.stringify(response.data));
        }
        return response.data;
      });
  }

  logout() {
    localStorage.removeItem('user');
  }

  register(user: User) {
    return axios.post(AUTH_API_URL + 'signup', {
      username: user.username,
      email: user.email,
      password: user.password
    });
  }
}

export default new AuthService();
