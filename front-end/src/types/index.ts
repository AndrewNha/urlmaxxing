export interface User {
  id: string;
  username: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}

export interface AuthCredentials {
  username: string;
  password: string;
}

export interface Bookmark {
  id: string;
  user_id: string;
  title: string;
  url: string;
  tags: string[];
  created_at: string;
}

export interface BookmarkInput {
  title: string;
  url: string;
  tags: string[];
}
