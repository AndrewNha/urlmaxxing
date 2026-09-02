import { TOKEN_KEY } from "@/lib/storage";

const API_URL = (import.meta.env.VITE_API_URL || "http://localhost:3000").replace(/\/$/, "");

interface ApiOptions extends Omit<RequestInit, "body"> {
  body?: unknown;
  authenticated?: boolean;
}

export class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

function getErrorMessage(status: number) {
  const messages: Record<number, string> = {
    400: "Check the submitted fields.",
    401: "Incorrect username or password.",
    403: "You do not have permission to do this.",
    404: "The requested item was not found.",
    409: "This username is already taken.",
    422: "The submitted data could not be validated.",
    500: "The server encountered an error. Try again.",
  };
  return messages[status] ?? "The request could not be completed.";
}

export async function api<T>(path: string, options: ApiOptions = {}): Promise<T> {
  const { authenticated = false, body, headers, ...requestOptions } = options;
  const token = localStorage.getItem(TOKEN_KEY);
  const response = await fetch(`${API_URL}${path}`, {
    ...requestOptions,
    headers: {
      ...(body !== undefined ? { "Content-Type": "application/json" } : {}),
      ...(authenticated && token ? { Authorization: `Bearer ${token}` } : {}),
      ...headers,
    },
    body: body !== undefined ? JSON.stringify(body) : undefined,
  }).catch(() => {
    throw new ApiError("Could not connect to the API. Check that the server is running.", 0);
  });

  const contentType = response.headers.get("content-type") ?? "";
  const data: unknown = contentType.includes("application/json")
    ? await response.json().catch(() => null)
    : await response.text().catch(() => "");

  if (!response.ok) {
    if (response.status === 401 && authenticated) {
      window.dispatchEvent(new Event("urlmaxxing:unauthorized"));
    }
    throw new ApiError(getErrorMessage(response.status), response.status);
  }

  return data as T;
}
