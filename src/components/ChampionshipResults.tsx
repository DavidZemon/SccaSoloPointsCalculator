import { Component, PropsWithoutRef } from 'react';
import { Accordion, Button, Card, Col, Row, Table } from 'react-bootstrap';
import {
  ChampionshipDriver,
  ChampionshipResults as ChampionshipResultsData,
  ChampionshipType,
  IndexedChampionshipResults,
} from '../models';
import {
  ChampionshipResultsParser,
  PaxService,
  toShortClassName,
} from '../services';
import { RamDownload } from './DownloadButton';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDownload } from '@fortawesome/free-solid-svg-icons';
import { EOL } from 'os';

interface ChampionshipResultsProps extends PropsWithoutRef<any> {
  paxService: PaxService;
  results?: ChampionshipResultsData;
}

interface ChampionshipResultsState {
  downloadName?: string;
  downloadData?: BlobPart;
}

export class ChampionshipResults extends Component<
  ChampionshipResultsProps,
  ChampionshipResultsState
> {
  constructor(props: Readonly<ChampionshipResultsProps>) {
    super(props);
    this.state = {};
  }

  render() {
    if (
      this.props.results &&
      Object.values(this.props.results).some((v) => !!v)
    ) {
      return [
        <Row key={0} className={'top-buffer'}>
          <Col>
            <h2>Championship Standings</h2>

            <Accordion>
              {this.renderClassChampionshipResults()}

              {Object.entries(this.props.results)
                // Class results need to be displayed separately
                .filter(([championshipType, _]) => championshipType !== 'Class')
                // Get some nice type hints going
                .map(
                  ([key, value]) =>
                    [key, value] as [
                      ChampionshipType,
                      IndexedChampionshipResults,
                    ],
                )
                .map(([championshipType, results]) => (
                  <Card key={championshipType}>
                    {this.renderCardHeader(championshipType, results.year)}

                    <Accordion.Collapse eventKey={championshipType}>
                      <Card.Body>
                        <Table striped hover borderless>
                          <thead>
                            <tr>
                              <th>Rank</th>
                              <th>Driver</th>
                              {new Array(results.drivers[0].points.length)
                                .fill(null)
                                .map((_, index) => (
                                  <th key={index}>Event #{index + 1}</th>
                                ))}
                              <th>Total Points</th>
                              <th>
                                Best{' '}
                                {ChampionshipResultsParser.calculateEventsToCount(
                                  results.drivers[0].points.length,
                                )}{' '}
                                of {results.drivers[0].points.length}
                              </th>
                            </tr>
                          </thead>

                          <tbody>
                            {results.drivers
                              // Reverse sort by doing `d2 - d1`, so top points shows up at the top
                              .sort((d1, d2) => d2.totalPoints - d1.totalPoints)
                              .map((driver, index) => (
                                <tr key={index}>
                                  <td>{index + 1}</td>
                                  <td>{driver.name}</td>
                                  {driver.points.map((points, index) => (
                                    <td key={index}>{points}</td>
                                  ))}

                                  <td>
                                    {driver.points.reduce(
                                      (sum, p) => sum + p,
                                      0,
                                    )}
                                  </td>

                                  <td>{driver.totalPoints}</td>
                                </tr>
                              ))}
                          </tbody>
                        </Table>
                      </Card.Body>
                    </Accordion.Collapse>
                  </Card>
                ))}
            </Accordion>
          </Col>
        </Row>,
        <RamDownload
          key={1}
          filename={this.state.downloadName}
          content={this.state.downloadData}
          contentType={'application/vnd.ms-excel'}
          downloadComplete={() =>
            this.setState({ downloadData: undefined, downloadName: undefined })
          }
        />,
      ];
    } else {
      return null;
    }
  }

  renderClassChampionshipResults(): JSX.Element | null {
    const results = this.props.results?.Class;
    if (results) {
      const championshipType: ChampionshipType = 'Class';
      const eventCount = Object.values(results.driversByClass)[0][0].points
        .length;
      return (
        <Card key={championshipType}>
          {this.renderCardHeader(championshipType, results.year)}

          <Accordion.Collapse eventKey={championshipType}>
            <Card.Body>
              <Table striped hover borderless>
                {Object.keys(results.driversByClass)
                  .sort()
                  .map((carClass) => [
                    <thead key={0}>
                      <tr>
                        <th colSpan={4 + eventCount}>{carClass}</th>
                      </tr>
                      <tr>
                        <th>Rank</th>
                        <th>Driver</th>
                        {new Array(eventCount).fill(null).map((_, index) => (
                          <th key={index}>Event #{index + 1}</th>
                        ))}
                        <th>Points</th>
                        <th>
                          Best{' '}
                          {ChampionshipResultsParser.calculateEventsToCount(
                            eventCount,
                          )}{' '}
                          of {eventCount}
                        </th>
                      </tr>
                    </thead>,
                    <tbody key={1}>
                      {results!.driversByClass[carClass]
                        // Reverse sort by doing `d2 - d1`, so top points shows up at the top
                        .sort((d1, d2) => d2.totalPoints - d1.totalPoints)
                        .map((driver, index) => (
                          <tr key={index}>
                            <td>{index + 1}</td>
                            <td>{driver.name}</td>
                            {driver.points.map((p, index) => (
                              <td key={index}>{p}</td>
                            ))}
                            <td>
                              {driver.points.reduce((sum, p) => sum + p, 0)}
                            </td>
                            <td>{driver.totalPoints}</td>
                          </tr>
                        ))}
                    </tbody>,
                  ])}
              </Table>
            </Card.Body>
          </Accordion.Collapse>
        </Card>
      );
    } else {
      return null;
    }
  }

  renderCardHeader(championshipType: ChampionshipType, year: number) {
    return (
      <Card.Header key={championshipType}>
        <Accordion.Toggle
          eventKey={championshipType}
          as={Button}
          variant={'link'}
        >
          {year} {championshipType}
        </Accordion.Toggle>
        <Button
          variant={'secondary'}
          onClick={() =>
            championshipType === 'Class'
              ? this.exportClassesAsCsv()
              : this.exportIndexAsCsv(championshipType)
          }
        >
          <FontAwesomeIcon className={'clickable'} icon={faDownload} />
        </Button>
      </Card.Header>
    );
  }

  private exportClassesAsCsv() {
    const results = this.props.results!.Class!;
    const { header } = this.startCsv();

    const rows = [
      ...header,
      ...Object.entries(results.driversByClass)
        .map(([carClass, drivers]) => {
          return [
            [`${toShortClassName(carClass)} - ${carClass}`],
            ...drivers
              .sort((d1, d2) => d2.totalPoints - d1.totalPoints)
              .map(ChampionshipResults.driverToCsv),
          ];
        })
        .flat(),
    ];
    this.setState({
      downloadData: rows.map((row) => `"${row.join('","')}"`).join(EOL),
      downloadName: `${results.year}_StL_Class_Championship.csv`,
    });
  }

  private exportIndexAsCsv(
    championshipType: Exclude<ChampionshipType, 'Class'>,
  ) {
    const results = this.props.results![championshipType]!;
    const { header } = this.startCsv();

    const rows = results.drivers
      .sort((d1, d2) => d2.totalPoints - d1.totalPoints)
      .map(ChampionshipResults.driverToCsv);

    this.setState({
      downloadData: [...header, ...rows]
        .map((row) => `"${row.join('","')}"`)
        .join(EOL),
      downloadName: `${results.year}_StL_${championshipType}_Championship.csv`,
    });
  }

  private startCsv() {
    const results = this.props.results!.Class!;
    const totalEventCount = Object.values(results.driversByClass)[0][0].points
      .length;
    const eventsToCount =
      ChampionshipResultsParser.calculateEventsToCount(totalEventCount);

    return {
      header: [
        [results.organization],
        [
          `${results.year} Class Championship -- Best ${eventsToCount} of ${totalEventCount}`,
        ],
        [],
        [
          'Rank',
          'Driver',
          ...new Array(totalEventCount)
            .fill(null)
            .map((_, index) => `Event #${index + 1}`),
          'Total Points',
          `Best ${eventsToCount} of ${totalEventCount}`,
        ],
      ],
    };
  }

  private static driverToCsv(
    driver: ChampionshipDriver,
    index: number,
  ): string[] {
    return [
      `${index + 1}`,
      driver.name,
      ...driver.points.map((p) => `${p}`),
      `${driver.points.reduce((sum, p) => sum + p, 0)}`,
      `${driver.totalPoints}`,
    ];
  }
}
