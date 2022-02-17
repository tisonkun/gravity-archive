/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2022-present tison <wander4096@gmail.com>
 */

package org.tisonkun.gravity.model.github.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Builder;
import lombok.Data;
import lombok.extern.jackson.Jacksonized;

@Jacksonized
@Builder
@Data
public class PullRequestReview {
    private final long id;
    @JsonProperty("node_id")
    private final String nodeId;
    private final String url;
    @JsonProperty("html_url")
    private final String htmlUrl;
    private final String body;
    @JsonProperty("author_association")
    private final String authorAssociation;
    private final User user;
    @JsonProperty("commit_id")
    private final String commitId;
    @JsonProperty("submitted_at")
    private final String submittedAt;
    private final String state;
}
