// ShardMasterAPI.kt
import io.ktor.application.*
import io.ktor.features.*
import io.ktor.http.*
import io.ktor.response.*
import io.ktor.routing.*
import io.ktor.serialization.*
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class Shard(val id: Int, val name: String, val config: Config)

@Serializable
data class Config(val database: String, val cache: String)

fun main() {
    embeddedServer(Netty, 8080) {
        install(ContentNegotiation) {
            json(Json {
                prettyPrint = true
                indent = "  "
            })
        }
        routing {
            get("/shards") {
                val shards = listOf(
                    Shard(1, "Shard 1", Config("db1", "cache1")),
                    Shard(2, "Shard 2", Config("db2", "cache2"))
                )
                call.respond(shards)
            }
            get("/shards/{id}") {
                val id = call.parameters["id"]!!.toInt()
                val shard = Shard(id, "Shard $id", Config("db$id", "cache$id"))
                call.respond(shard)
            }
        }
    }.start(wait = true)
}
