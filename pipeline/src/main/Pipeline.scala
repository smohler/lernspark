import com.amazonaws.services.s3.AmazonS3ClientBuilder
import org.apache.spark.sql.SparkSession

object Pipeline {
  def main(args: Array[String]): Unit = {
    val spark = SparkSession.builder()
      .appName("lernspark")
      .getOrCreate()

    val inputPath = sys.env.getOrElse("INPUT_PATH", "s3://default-input-bucket/path/to/parquet/files")
    val outputPath = sys.env.getOrElse("OUTPUT_PATH", "s3://default-output-bucket/output")

    // Create the output S3 bucket if it doesn't exist
    createS3BucketIfNotExists(outputPath)

    // Read parquet files from S3 into a DataFrame
    val parquetDF = spark.read.parquet(inputPath)

    // Register the DataFrame as a temporary view
    parquetDF.createOrReplaceTempView("parquet_data")

    // Read the pipeline SQL from the root directory
    val pipelineSQL = scala.io.Source.fromFile("pipeline.sql").mkString
    val jobs = pipelineSQL.split(";").map(_.trim).filter(_.nonEmpty)

    // Execute each job in the pipeline on the parquet data
    jobs.foreach { job =>
      val result = spark.sql(job)
      // Save the result of each job to S3 if needed
      result.write.format("parquet").save(s"$outputPath/${job.takeWhile(_ != ' ')}")
    }

    spark.stop()
  }

  def createS3BucketIfNotExists(s3Path: String): Unit = {
    val s3Client = AmazonS3ClientBuilder.defaultClient()
    val bucketName = s3Path.split("/")(2)

    if (!s3Client.doesBucketExistV2(bucketName)) {
      s3Client.createBucket(bucketName)
      println(s"Created S3 bucket: $bucketName")
    } else {
      println(s"S3 bucket already exists: $bucketName")
    }
  }
}
