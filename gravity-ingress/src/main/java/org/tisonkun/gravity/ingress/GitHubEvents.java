/*
 * SPDX-License-Identifier: Apache-2.0
 * Copyright 2022-present tison <wander4096@gmail.com>
 */
package org.tisonkun.gravity.ingress;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.List;
import org.tisonkun.gravity.model.github.Activity;
import org.tisonkun.gravity.model.github.event.ForkEvent;
import org.tisonkun.gravity.model.github.event.IssueCommentEvent;
import org.tisonkun.gravity.model.github.event.IssuesEvent;
import org.tisonkun.gravity.model.github.event.PullRequestEvent;
import org.tisonkun.gravity.model.github.event.PullRequestReviewCommentEvent;
import org.tisonkun.gravity.model.github.event.PullRequestReviewEvent;
import org.tisonkun.gravity.model.github.event.WatchEvent;

public class GitHubEvents {
    public static void main(String[] args) throws Exception {
        final var client = HttpClient.newHttpClient();
        final var request = HttpRequest.newBuilder()
            .uri(URI.create("https://api.github.com/repos/pingcap/tidb/events"))
            .header("Accept", "application/vnd.github.v3+json")
            .method("GET", HttpRequest.BodyPublishers.noBody())
            .build();

        final var response = client.send(request, HttpResponse.BodyHandlers.ofString());
        final var body = response.body();
        final var mapper = new ObjectMapper().configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);
        final var activities = mapper.readValue(body, new TypeReference<List<Activity>>() {
        });

        final var eventTypes = List.of(
            ForkEvent.class,
            IssueCommentEvent.class,
            IssuesEvent.class,
            PullRequestEvent.class,
            PullRequestReviewEvent.class,
            PullRequestReviewCommentEvent.class,
            WatchEvent.class
        );

        for (var eventType : eventTypes) {
            final var events = activities.stream().filter(activity -> activity.getType().equals(eventType.getSimpleName())).toList();
            final var matched = events.stream().allMatch(activity -> activity.getPayload().getClass().equals(eventType));
            if (matched) {
                if (eventType.equals(ForkEvent.class)) {
                    System.out.println(mapper.writerWithDefaultPrettyPrinter().writeValueAsString(events));
                }
            } else {
                throw new Exception("Mismatched: %s!".formatted(eventType.getSimpleName()));
            }
        }
    }
}
