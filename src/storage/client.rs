use crate::environment::environment;
use super::{bucket, bucket::Bucket};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, primitives::ByteStream};

pub struct Storage {
    client: Client,
}

impl Storage {
    pub async fn new() -> Self {
        let environment = environment();
        let credentials = aws_sdk_s3::config::Credentials::new(
            environment.storage.access_key.clone(),
            environment.storage.secret_key.clone(),
            None,
            None,
            "credentials",
        );
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(environment.storage.region.clone()))
            .credentials_provider(credentials)
            .load()
            .await;
        let s3_config = aws_sdk_s3::config::Builder::from(&config)
            .endpoint_url(environment.storage.endpoint.clone())
            .force_path_style(true)
            .build();

        let client = Client::from_conf(s3_config);

        let storage = Storage { client };
        storage.setup_buckets().await;

        storage
    }

    async fn make_bucket_public(&self, bucket: Bucket) {
        let policy = serde_json::json!({
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Effect": "Allow",
                    "Principal": "*",
                    "Action": "s3:GetObject",
                    "Resource": format!("arn:aws:s3:::{}/*", bucket.as_str()),
                }
            ]
        });

        self.client
            .put_bucket_policy()
            .bucket(bucket.as_str())
            .policy(policy.to_string())
            .send()
            .await
            .unwrap();
    }

    async fn create_bucket(&self, bucket: Bucket) {
        self.client
            .create_bucket()
            .bucket(bucket.as_str())
            .send()
            .await
            .unwrap();

    }

    async fn setup_buckets(&self) {
        let buckets = bucket::get_all();
        for bucket in buckets {
            self.create_bucket(bucket.clone()).await;
            if bucket.is_public() { self.make_bucket_public(bucket).await; }
        }
    }

    pub async fn upload_file(&self, bucket: Bucket, file_path: &str, content_type: &str, content: ByteStream) {
        self.client
            .put_object()
            .key(file_path)
            .bucket(bucket.as_str())
            .content_type(content_type)
            .body(content)
            .send()
            .await
            .unwrap();
    }
}
