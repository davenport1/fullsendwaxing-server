# Api Backend For Full Send Waxing

Full send waxing backend api build using axum rust web framework and sea orm

## API Endpoints

### `GET /`
- **Description**: Returns a simple "Hello, World!" message.
- **Response**: `200 OK` with plain text message.

### `POST /post_string`
- **Description**: Echoes back the string sent in the request body.
- **Request Body**: Plain text string.
- **Response**: `200 OK` with the received string.

### `POST /post_json`
- **Description**: Mirrors back the JSON sent in the request body.
- **Request Body**: JSON object.
- **Response**: `200 OK` with the received JSON.

### `GET /path_variables/:id`
- **Description**: Demonstrates the use of path variables.
- **Path Variable**: `id` (any string or number).
- **Response**: `200 OK` with details about the received path variable.

### `GET /path_variabled/15`
- **Description**: A route with a hard-coded path variable.
- **Response**: `200 OK` with specific response for path variable `15`.

### `GET /query_params`
- **Description**: Returns the query parameters sent in the request.
- **Query Parameters**: Any.
- **Response**: `200 OK` with the received query parameters.

### `GET /mirror_user_agent`
- **Description**: Returns the `User-Agent` header from the request.
- **Response**: `200 OK` with the `User-Agent` value.

### `GET /mirror_custom_header`
- **Description**: Demonstrates the use of custom headers.
- **Custom Header**: Any custom header sent in the request.
- **Response**: `200 OK` with the received custom header value.

### `GET /middleware_message`
- **Description**: Utilizes middleware to append a message.
- **Response**: `200 OK` with a message from middleware.

### `GET /read_middleware_custom_header`
- **Description**: Reads a custom header set by middleware.
- **Response**: `200 OK` with the custom header value set by middleware.

### `GET /always_errors`
- **Description**: This endpoint always returns an error.
- **Response**: `500 Internal Server Error`.

### `POST /returns_201`
- **Description**: Returns a `201 Created` status code.
- **Response**: `201 Created`.

## Middleware
- **CORS Middleware**: Applied globally, allowing `GET` and `POST` methods from any origin.
- **Custom Middleware for Header Setting**: Applied to the `/read_middleware_custom_header` route.

## Shared Data
- **`SharedData` Structure**: Holds shared data accessible across routes. Example field: `message`.

## Additional Information
- **Error Handling**: Standard HTTP status codes are used to indicate errors.
- **Versioning**: This documentation is for version 1.0.0 of the API. Future versions may have different endpoints or behaviors.

## Examples and Usage
ToDo
