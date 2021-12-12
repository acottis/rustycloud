### How to Deploy in Google Cloud Run###

```gcloud run deploy --cpu=1 --memory=512Mi --max-instances=1 --min-instances=0 --concurrency=200 --region=europe-north1 --source=rustycloud```