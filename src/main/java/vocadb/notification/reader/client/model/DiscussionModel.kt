package vocadb.notification.reader.client.model

import java.time.LocalDateTime

data class DiscussionTopicContract constructor(
    val author: UserForApiContract? = null,
    val commentCount: Int? = null,
    val comments: Collection<CommentForApiContract>? = null,
    val content: String? = null,
    val created: LocalDateTime? = null,
    val folderId: Int? = null,
    val id: Int? = null,
    val lastComment: CommentForApiContract? = null,
    val locked: Boolean? = null,
    val name: String? = null
)

data class DiscussionFolderContract constructor(
    val description: String? = null,
    val id: Int? = null,
    val lastTopicAuthor: UserForApiContract? = null,
    val lastTopicDate: LocalDateTime? = null,
    val name: String? = null,
    val topicCount: Int? = null
)

data class EntryRefContract constructor(
    val entryType: EntryType? = null,
    val id: Int? = null
)

data class CommentForApiContract constructor(
    val author: UserForApiContract? = null,
    val authorName: String? = null,
    val created: LocalDateTime? = null,
    val entry: EntryRefContract? = null,
    val id: Int? = null,
    val message: String? = null
)
