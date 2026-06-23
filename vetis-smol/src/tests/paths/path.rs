mod handler_path_tests {
    use http::StatusCode;
    use smol_macros::test;
    use vetis::{
        errors::{HandlerError, VetisError, VirtualHostError},
        virtual_host::{handler_fn, path::Path},
        Response,
    };

    use crate::virtual_host::path::HandlerPath;

    #[test]
    fn test_handler_path_builder_default() {
        let builder = HandlerPath::builder();
        let _ = builder;
    }

    #[test]
    fn test_handler_path_builder_build_empty_uri() {
        let handler = handler_fn(|_request| async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .text("Hello"))
        });

        let builder = HandlerPath::builder()
            .uri("")
            .handler(handler);

        let result = builder.build();

        assert!(result.is_err());
        if let Err(VetisError::VirtualHost(VirtualHostError::Handler(HandlerError::Uri(msg)))) = result {
            assert_eq!(msg, "URI cannot be empty");
        } else {
            panic!("Expected URI error");
        }
    }

    #[test]
    fn test_handler_path_builder_build_no_handler() {
        let builder = HandlerPath::builder().uri("/test");

        let result = builder.build();

        assert!(result.is_err());
        if let Err(VetisError::VirtualHost(VirtualHostError::Handler(HandlerError::Handler(msg)))) = result {
            assert_eq!(msg, "Handler must be set");
        } else {
            panic!("Expected handler error");
        }
    }

    #[test]
    fn test_handler_path_builder_build_success() {
        let handler = handler_fn(|_request| async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .text("Hello"))
        });

        let builder = HandlerPath::builder()
            .uri("/test")
            .handler(handler);

        let result = builder.build();

        assert!(result.is_ok());
        let _ = result.unwrap();
    }

    #[test]
    fn test_handler_path_uri() {
        let handler = handler_fn(|_request| async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .text("Hello"))
        });

        let result = HandlerPath::builder()
            .uri("/api/test")
            .handler(handler)
            .build();

        assert!(result.is_ok());
        let handler_path = result.unwrap();
        assert_eq!(handler_path.uri(), "/api/test");
    }

    #[test]
    fn test_handler_path_builder_multiple_uri() {
        let handler = handler_fn(|_request| async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .text("Hello"))
        });

        let result = HandlerPath::builder()
            .uri("/api/v1/users")
            .handler(handler)
            .build();

        assert!(result.is_ok());
        let handler_path = result.unwrap();
        assert_eq!(handler_path.uri(), "/api/v1/users");
    }

    #[test]
    fn test_handler_path_builder_uri_with_query() {
        let handler = handler_fn(|_request| async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .text("Hello"))
        });

        let result = HandlerPath::builder()
            .uri("/search?q=test")
            .handler(handler)
            .build();

        assert!(result.is_ok());
        let handler_path = result.unwrap();
        assert_eq!(handler_path.uri(), "/search?q=test");
    }

    #[test]
    fn test_handler_path_builder_root_uri() {
        let handler = handler_fn(|_request| async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .text("Hello"))
        });

        let result = HandlerPath::builder()
            .uri("/")
            .handler(handler)
            .build();

        assert!(result.is_ok());
        let handler_path = result.unwrap();
        assert_eq!(handler_path.uri(), "/");
    }

    #[test]
    fn test_handler_path_builder_complex_uri() {
        let handler = handler_fn(|_request| async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .text("Hello"))
        });

        let result = HandlerPath::builder()
            .uri("/api/v1/users/123/posts/456")
            .handler(handler)
            .build();

        assert!(result.is_ok());
        let handler_path = result.unwrap();
        assert_eq!(handler_path.uri(), "/api/v1/users/123/posts/456");
    }
}
