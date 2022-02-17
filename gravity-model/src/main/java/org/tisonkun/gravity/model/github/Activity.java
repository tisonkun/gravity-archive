/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2022-present tison <wander4096@gmail.com>
 */
package org.tisonkun.gravity.model.github;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import lombok.Builder;
import lombok.Data;
import lombok.extern.jackson.Jacksonized;
import org.tisonkun.gravity.model.github.event.Event;
import org.tisonkun.gravity.model.github.event.ForkEvent;
import org.tisonkun.gravity.model.github.event.IssueCommentEvent;
import org.tisonkun.gravity.model.github.event.IssuesEvent;
import org.tisonkun.gravity.model.github.event.PullRequestEvent;
import org.tisonkun.gravity.model.github.event.PullRequestReviewCommentEvent;
import org.tisonkun.gravity.model.github.event.PullRequestReviewEvent;
import org.tisonkun.gravity.model.github.event.UnknownEvent;
import org.tisonkun.gravity.model.github.event.WatchEvent;
import org.tisonkun.gravity.model.github.model.OrganizationLite;
import org.tisonkun.gravity.model.github.model.RepositoryLite;
import org.tisonkun.gravity.model.github.model.UserLite;

@Jacksonized
@Builder
@Data
public class Activity {
    private final String id;
    private final String type;
    @JsonProperty("created_at")
    private final String createdAt;
    private final UserLite actor;
    @JsonProperty("org")
    private final OrganizationLite organization;
    @JsonProperty("repo")
    private final RepositoryLite repository;

    @JsonTypeInfo(
        use = JsonTypeInfo.Id.NAME,
        include = JsonTypeInfo.As.EXTERNAL_PROPERTY,
        property = "type",
        defaultImpl = UnknownEvent.class)
    @JsonSubTypes(value = {
        @JsonSubTypes.Type(value = ForkEvent.class, name = "ForkEvent"),
        @JsonSubTypes.Type(value = IssueCommentEvent.class, name = "IssueCommentEvent"),
        @JsonSubTypes.Type(value = IssuesEvent.class, name = "IssuesEvent"),
        @JsonSubTypes.Type(value = PullRequestEvent.class, name = "PullRequestEvent"),
        @JsonSubTypes.Type(value = PullRequestReviewEvent.class, name = "PullRequestReviewEvent"),
        @JsonSubTypes.Type(value = PullRequestReviewCommentEvent.class, name = "PullRequestReviewCommentEvent"),
        @JsonSubTypes.Type(value = WatchEvent.class, name = "WatchEvent"),
    })
    private final Event payload;
}
