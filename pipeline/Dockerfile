FROM openjdk:8-jdk-alpine

# Install Scala
RUN apk add --no-cache scala

# Install Apache Spark
RUN wget https://downloads.apache.org/spark/spark-3.1.2/spark-3.1.2-bin-hadoop3.2.tgz
RUN tar -xzf spark-3.1.2-bin-hadoop3.2.tgz
RUN mv spark-3.1.2-bin-hadoop3.2 /usr/local/spark

# Set environment variables
ENV SPARK_HOME=/usr/local/spark
ENV PATH=$PATH:$SPARK_HOME/bin

# Copy application files
COPY src /app/src
COPY build.sbt /app/build.sbt

# Build application
WORKDIR /app
RUN sbt assembly

# Expose port for Spark UI
EXPOSE 4040

# Start Spark master and worker
CMD ["spark-3.1.2-bin-hadoop3.2/sbin/start-master.sh"]
CMD ["spark-3.1.2-bin-hadoop3.2/sbin/start-slave.sh", "spark://localhost:7077"]

# Run application
CMD ["spark-submit", "--class", "Main", "--master", "spark://localhost:7077", "target/scala-2.12/app.jar"]
