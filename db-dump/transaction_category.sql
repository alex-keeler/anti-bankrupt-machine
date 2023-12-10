-- MySQL dump 10.13  Distrib 5.7.42, for osx10.18 (x86_64)
--
-- Host: localhost    Database: budgetmanager
-- ------------------------------------------------------
-- Server version	5.7.42

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `transaction_category`
--

DROP TABLE IF EXISTS `transaction_category`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `transaction_category` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `title` varchar(100) DEFAULT NULL,
  `category_group_id` bigint(20) NOT NULL,
  `user_id` bigint(20) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_tc_tcg_t` (`category_group_id`,`title`),
  KEY `fk_tc_u` (`user_id`),
  CONSTRAINT `fk_tc_u` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`),
  CONSTRAINT `transaction_category_FK` FOREIGN KEY (`category_group_id`) REFERENCES `transaction_category_group` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=51 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `transaction_category`
--

LOCK TABLES `transaction_category` WRITE;
/*!40000 ALTER TABLE `transaction_category` DISABLE KEYS */;
INSERT INTO `transaction_category` VALUES (1,NULL,1,1),(2,'Electric',2,1),(3,'Gas',2,1),(4,'Internet',2,1),(5,'Renter’s Insurance',2,1),(6,'Gas',3,1),(7,'Car Insurance',3,1),(8,'Car Registration',3,1),(9,'Oil Change',3,1),(10,'Car Maintenance',3,1),(11,NULL,4,1),(12,'Routine',5,1),(13,'Hospital',5,1),(14,'Veterinary',5,1),(15,'Medication',5,1),(16,'Emergency',6,1),(17,'Travel',6,1),(18,'Event',6,1),(19,'Stocks',7,1),(20,'Bonds',7,1),(21,'Retirement Account',7,1),(22,'Real Estate',7,1),(23,'Food',8,1),(24,'Drinks',8,1),(25,'Snacks/Treats',8,1),(26,'Concert',9,1),(27,'Theater',9,1),(28,'Movie',9,1),(29,'Miscellaneous',9,1),(30,'Gaming',10,1),(31,'Clothing',10,1),(32,'Media',10,1),(33,'Musical',10,1),(34,'Hardware',10,1),(35,'Software',10,1),(36,'Accessories',10,1),(37,'Services',10,1),(38,'Furniture/Decorations',10,1),(39,'Miscellaneous',10,1),(40,'Transfer',11,1),(41,'Transportation',12,1),(42,'Shopping',12,1),(43,'Hotel',12,1),(44,'Activities',12,1),(45,'Miscellaneous',12,1),(46,'Parking',13,1),(47,'Fees',13,1),(48,NULL,14,1),(49,'Food',10,1),(50,'External Financing',10,1);
/*!40000 ALTER TABLE `transaction_category` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2023-10-28 23:25:42
