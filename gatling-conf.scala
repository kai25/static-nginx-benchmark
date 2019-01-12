package computerdatabase

import io.gatling.core.Predef._
import io.gatling.http.Predef._
import scala.concurrent.duration._

class MySimulation extends Simulation {

  val httpProtocol = http
    .baseUrl("http://localhost:8080/data/med.txt")

  val scn = scenario("My")
    .exec(http("request_1")
      .get(""))

  setUp(scn.inject(
	constantUsersPerSec(10) during (10 seconds),
        rampUsersPerSec(10) to 100 during (5 seconds),
        constantUsersPerSec(100) during (30 seconds),
        rampUsersPerSec(100) to 250 during (15 seconds),
        constantUsersPerSec(400) during (30 seconds),
        rampUsersPerSec(400) to 1000 during (10 seconds),
        constantUsersPerSec(1000) during (1 minutes),
        constantUsersPerSec(200) during (20 seconds),
        constantUsersPerSec(20) during (10 seconds)
  ).protocols(httpProtocol))
}
