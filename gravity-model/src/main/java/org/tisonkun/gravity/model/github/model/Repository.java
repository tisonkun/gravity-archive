/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2022-present tison <wander4096@gmail.com>
 */
package org.tisonkun.gravity.model.github.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;
import lombok.Builder;
import lombok.Data;
import lombok.extern.jackson.Jacksonized;

@Jacksonized
@Builder
@Data
public class Repository {
    private final int id;
    @JsonProperty("node_id")
    private final String nodeId;
    private final String url;
    @JsonProperty("html_url")
    private final String htmlUrl;
    private final String name;
    @JsonProperty("full_name")
    private final String fullName;
    @JsonProperty("private")
    private final boolean isPrivate;
    private final User owner;
    private final String description;
    private final boolean fork;
    private final String homepage;
    private final long size;
    @JsonProperty("stargazers_count")
    private final long stargazersCount;
    @JsonProperty("watchers_count")
    private final long watchersCount;
    private final License license;
    private final String language;
    @JsonProperty("has_issues")
    private final boolean hasIssues;
    @JsonProperty("has_projects")
    private final boolean hasProjects;
    @JsonProperty("has_downloads")
    private final boolean hasDownloads;
    @JsonProperty("has_wiki")
    private final boolean hasWiki;
    @JsonProperty("has_pages")
    private final boolean hasPages;
    @JsonProperty("forks_count")
    private final long forksCount;
    @JsonProperty("mirror_url")
    private final String mirrorUrl;
    private final boolean archived;
    private final boolean disabled;
    @JsonProperty("open_issues_count")
    private final long openIssuesCount;
    @JsonProperty("allow_forking")
    private final boolean allowForking;
    @JsonProperty("is_template")
    private final boolean isTemplate;
    private final List<String> topics;
    private final String visibility;
    private final long forks;
    private final long watchers;
    @JsonProperty("open_issues")
    private final long openIssues;
    @JsonProperty("default_branch")
    private final String defaultBranch;
}
