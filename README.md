# lernspark
Have you ever wondered the the fuck Apache Spark, Docker, CI/CD, and modern big data architectures look are? Me too!
To better understand I made this application that does a few things to help make it clear.

1. Make some random [parquet data](https://parquet.apache.org) quickly defined by you.
2. Upload that data to an S3 bucket 
3. Define a pipeline on the data
4. Execute that pipline on a subset of the S3 data for testing
5. Dockerize your pipeline to make an image!
6. Upload to [ECR](https://docs.aws.amazon.com/AmazonECR/latest/userguide/what-is-ecr.html?pg=ln&sec=hs)
7. Configure a Step Function to run the container for new uploads to S3 bucket or rerun if the container version changes
8. Make changes to the pipeline? Run a [GitHub Action](https://docs.github.com/en/actions/learn-github-actions/understanding-github-actions) when you merge to `main`

## Installation
To install `lernspark` you should just for now clone the repo and run some of the scripts.
```
cd ~
git clone https://github.com/smohler/lernspark.git
cd lernspark
chmod +x ~/lernspark/scripts/macOS/bootstrap.sh
~/lernspark/scripts/macOS/bootstrap.sh
```
After `bootstrap.sh` runs you will have some commands loaded in your environment to help explor and interact with lernspark.

