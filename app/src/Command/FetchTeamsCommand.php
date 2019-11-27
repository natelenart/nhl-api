<?php

namespace App\Command;

use App\Service\Fetcher;
use Symfony\Component\Console\Command\Command;
use Symfony\Component\Console\Input\InputArgument;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Input\InputOption;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Console\Style\SymfonyStyle;

class FetchTeamsCommand extends Command
{
    protected static $defaultName = 'app:fetch:teams';

    /**
     * @var Fetcher $fetcher
     */
    private $fetcher;

    public function __construct(Fetcher $fetcher)
    {
        $this->fetcher = $fetcher;

        parent::__construct();
    }

    protected function configure()
    {
        $this
            ->setDescription('Fetch teams from the NHL API')
        ;
    }

    protected function execute(InputInterface $input, OutputInterface $output): int
    {
        $io = new SymfonyStyle($input, $output);

        $this->fetcher->fetchTeams();

        $io->success('Teams fetched from API');

        return 0;
    }
}
