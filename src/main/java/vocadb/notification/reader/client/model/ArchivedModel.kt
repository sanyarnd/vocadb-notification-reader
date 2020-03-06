package vocadb.notification.reader.client.model

data class ArchivedWebLinkContract constructor(
    val category: WebLinkCategory? = null,
    val description: String? = null,
    val url: String? = null
)

data class ArchivedObjectVersionForApiContract constructor(
    val changedFields: Collection<String>? = null,
    val id: Int? = null,
    val notes: String? = null,
    val version: Int? = null
)
