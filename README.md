# Api Backend For Full Send Waxing

Full send waxing backend api build using axum rust web framework and sea orm

## API Endpoints

### `GET /appointments/{id}`
- **Description**: Fetches the appointment for the specified ID
- **Response**: Appointment JSON.

### `POST /appointments/`
- **Description**: Saves an appointment 
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
