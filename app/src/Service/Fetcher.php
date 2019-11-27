<?php

namespace App\Service;

use App\Entity\Team;
use Doctrine\ORM\EntityManagerInterface;
use GuzzleHttp\Client;

class Fetcher
{
    const URL_TEAMS = "teams";

    /**
     * @var EntityManagerInterface
     */
    private $em;

    /**
     * @var Client
     */
    private $client;

    public function __construct(EntityManagerInterface $em)
    {
        $this->em = $em;
        $this->client = new Client([
            'base_uri' => 'https://statsapi.web.nhl.com/api/v1/',
        ]);
    }

    public function fetchTeams()
    {
        $resp = $this->client->request('GET', self::URL_TEAMS);
        $body = (string) $resp->getBody();
        $contents = json_decode($body, true);

        foreach ($contents['teams'] as $team_data) {
            $team = (new Team())
                ->setNhlId($team_data['id'])
                ->setAbbreviation($team_data['abbreviation'])
                ->setLocation($team_data['locationName'])
                ->setName($team_data['teamName'])
            ;
            $this->em->persist($team);
        }

        $this->em->flush();
    }
}