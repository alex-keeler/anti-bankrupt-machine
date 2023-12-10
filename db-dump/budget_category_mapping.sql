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
-- Table structure for table `budget_category_mapping`
--

DROP TABLE IF EXISTS `budget_category_mapping`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `budget_category_mapping` (
  `id` bigint(20) NOT NULL AUTO_INCREMENT,
  `budget_id` bigint(20) NOT NULL,
  `category_id` bigint(20) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `fk_bcm_b` (`budget_id`),
  KEY `fk_bcm_c` (`category_id`),
  CONSTRAINT `fk_bcm_b` FOREIGN KEY (`budget_id`) REFERENCES `budget` (`id`),
  CONSTRAINT `fk_bcm_c` FOREIGN KEY (`category_id`) REFERENCES `transaction_category` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=28 DEFAULT CHARSET=utf8mb4;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `budget_category_mapping`
--

LOCK TABLES `budget_category_mapping` WRITE;
/*!40000 ALTER TABLE `budget_category_mapping` DISABLE KEYS */;
INSERT INTO `budget_category_mapping` VALUES (1,1,1),(2,2,2),(3,2,3),(4,3,4),(5,4,7),(6,5,5),(7,6,9),(8,7,11),(9,8,6),(10,9,23),(11,9,24),(12,9,25),(13,10,26),(14,10,27),(15,10,28),(16,10,29),(17,11,30),(18,11,31),(19,11,32),(20,11,33),(21,11,34),(22,11,35),(23,11,36),(24,11,37),(25,11,38),(26,11,39),(27,15,50);
/*!40000 ALTER TABLE `budget_category_mapping` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2023-10-28 23:26:24
