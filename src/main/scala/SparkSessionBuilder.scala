import org.apache.spark.sql.SparkSession

object SparkSessionBuilder {
  def build(): SparkSession = {
    SparkSession.builder()
      .appName("My Spark Application")
      .config("spark.master", "local")
      .getOrCreate()
  }
}
